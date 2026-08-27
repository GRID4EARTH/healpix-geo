import init, * as healpixGeo from "../pkg/index.js";
import { Coordinate, Ellipsoid, Grid } from "../pkg/index.js";
import { describe, expect, test } from "vitest";

const wgs84 = {
  semi_major_axis: 6378137.0,
  inverse_flattening: 298.257223563,
};

describe("new Grid", () => {
  test("basic construction", () => {
    const grid = new Grid({ scheme: "nested", level: 4 });
    expect(grid.scheme).to.equal("nested");
    expect(grid.level).to.equal(4);
    expect(grid.nside).to.equal(16);
    expect(grid.ellipsoid.isSphere).to.equal(true);
  });

  test("accepts an ellipsoid definition", () => {
    const grid = new Grid({
      scheme: "nested",
      level: 4,
      ellipsoid: wgs84,
    });
    expect(grid.ellipsoid.isSphere).to.equal(false);
    expect(grid.ellipsoid.semiMajorAxis).to.equal(wgs84.semi_major_axis);
  });

  test("flattened ellipsoid getters avoid the handle allocation", () => {
    // `grid.ellipsoid` mints a new handle (allocation + finalizer) on every
    // read; these return plain numbers
    const grid = new Grid({
      scheme: "nested",
      level: 4,
      ellipsoid: wgs84,
    });

    const handle = grid.ellipsoid;
    expect(grid.semiMajorAxis).to.equal(handle.semiMajorAxis);
    expect(grid.flattening).to.equal(handle.flattening);
    expect(grid.isSphere).to.equal(handle.isSphere);
    expect(grid.isSphere).to.equal(false);
    handle.free();
  });

  test("rejects invalid options", () => {
    // @ts-expect-error level is required
    expect(() => new Grid({ scheme: "nested" })).to.throw(/level/);
    expect(() => new Grid({ scheme: "nested", level: 30 })).to.throw(
      /`level` must be in \[0, 29\]/,
    );
    // @ts-expect-error scheme must be one of the three names
    expect(() => new Grid({ scheme: "bogus", level: 4 })).to.throw();
  });
});

describe("Grid input validation", () => {
  // every one of these used to be an unrecoverable wasm trap, which poisons
  // the whole module instance rather than throwing
  test("rejects cell ids out of range at the grid's level", () => {
    for (const scheme of ["nested", "ring"] as const) {
      const grid = new Grid({ scheme, level: 4 });
      // 12 * 4^4 == 3072
      expect(() => grid.vertex(3072n, 0.5, 0.5)).to.throw(/out of range/);
      expect(() => grid.vertex(2n ** 64n - 1n, 0.5, 0.5)).to.throw(
        /out of range/,
      );
      expect(grid.vertex(3071n, 0.5, 0.5).lon).to.be.a("number");
    }
  });

  test("rejects malformed zuniq cell ids", () => {
    const grid = new Grid({ scheme: "zuniq", level: 4 });

    expect(() => grid.vertex(0n, 0.5, 0.5)).to.throw(/zuniq/);
    expect(() => grid.vertex(2n, 0.5, 0.5)).to.throw(/zuniq/);
    expect(() => grid.vertex(2n ** 64n - 1n, 0.5, 0.5)).to.throw(/zuniq/);
  });

  test("rejects z-order coordinates outside the base cell", () => {
    for (const scheme of ["nested", "ring", "zuniq"] as const) {
      const grid = new Grid({ scheme, level: 1 });
      expect(() => grid.bitCombine(0, 2)).to.throw(/out of range/);
      expect(() => grid.bitCombine(256, 0)).to.throw(/out of range/);
      expect(grid.bitCombine(1, 1)).to.be.a("bigint");
    }
  });

  test("rejects z-order coordinates wasm-bindgen would have coerced", () => {
    // declared as `u32`, these arrived as `(0, 0)` / `(1, 1)` after ToUint32
    // and passed the range check they were supposed to fail
    const grid = new Grid({ scheme: "nested", level: 4 });

    expect(() => grid.bitCombine(2 ** 32, 0)).to.throw(/must be in/);
    expect(() => grid.bitCombine(1.9, 1)).to.throw(/integer/);
    expect(() => grid.bitCombine(NaN, 0)).to.throw(/integer/);
    expect(grid.bitCombine(1, 1)).to.be.a("bigint");
  });

  test("rejects vertex offsets outside [0, 1]", () => {
    const grid = new Grid({ scheme: "nested", level: 4 });

    expect(() => grid.vertex(164n, NaN, 0)).to.throw(/\[0, 1\]/);
    expect(() => grid.vertex(164n, 0, NaN)).to.throw(/\[0, 1\]/);
    expect(() => grid.vertex(164n, Infinity, 0)).to.throw(/\[0, 1\]/);
    expect(() => grid.vertex(164n, -0.1, 0)).to.throw(/\[0, 1\]/);
    expect(() => grid.vertex(164n, 0, 1.5)).to.throw(/\[0, 1\]/);
    expect(() => grid.vertex(164n, 100, 100)).to.throw(/\[0, 1\]/);

    // the boundaries themselves are legal
    expect(grid.vertex(164n, 0, 0).lon).to.be.a("number");
    expect(grid.vertex(164n, 1, 1).lon).to.be.a("number");
  });

  test("the module stays usable after a rejected call", () => {
    const grid = new Grid({ scheme: "ring", level: 1 });
    expect(() => grid.bitCombine(256, 0)).to.throw();
    expect(() => grid.vertex(0n, NaN, 0)).to.throw();
    expect(() => grid.vertex(2n ** 64n - 1n, 0.5, 0.5)).to.throw();
    expect(grid.vertex(0n, 0.5, 0.5).lon).to.be.a("number");
  });
});

describe("Grid parity with the scheme statics", () => {
  test("nested", () => {
    const grid = new Grid({
      scheme: "nested",
      level: 4,
      ellipsoid: wgs84,
    });
    const ellipsoid = Ellipsoid.from(wgs84);

    const vertex = grid.vertex(164n, 0.25, 0.75);
    const staticVertex = healpixGeo.nested.vertex(
      164n,
      4,
      0.25,
      0.75,
      ellipsoid,
    );
    expect(vertex.lon).to.equal(staticVertex.lon);
    expect(vertex.lat).to.equal(staticVertex.lat);

    expect(grid.bitCombine(3, 5)).to.equal(
      healpixGeo.nested.bitCombine(4, 3, 5),
    );
  });

  test("ring", () => {
    const grid = new Grid({ scheme: "ring", level: 4 });
    const sphere = Ellipsoid.from(null);

    const vertex = grid.vertex(164n, 0.5, 0.5);
    const staticVertex = healpixGeo.ring.vertex(164n, 4, 0.5, 0.5, sphere);
    expect(vertex.lon).to.equal(staticVertex.lon);
    expect(vertex.lat).to.equal(staticVertex.lat);
  });

  test("zuniq hides the signature difference", () => {
    // the zuniq statics take no level for vertex; the grid object exposes the
    // exact same call shape for all three schemes
    const grid = new Grid({ scheme: "zuniq", level: 0 });
    const sphere = Ellipsoid.from(null);

    const cell = healpixGeo.zuniq.bitCombine(0, 0, 0);

    const vertex = grid.vertex(cell, 1.0, 1.0);
    const staticVertex = healpixGeo.zuniq.vertex(cell, 1.0, 1.0, sphere);
    expect(vertex.lon).to.equal(staticVertex.lon);
    expect(vertex.lat).to.equal(staticVertex.lat);
  });

  test("uniform loop over all schemes", () => {
    // the motivating gridlook use case: one code path for every scheme
    for (const scheme of ["nested", "ring", "zuniq"] as const) {
      const grid = new Grid({ scheme, level: 2 });
      const cell = grid.bitCombine(1, 2);
      const vertex: Coordinate = grid.vertex(cell, 0.5, 0.5);
      expect(vertex.lon).to.be.a("number");
      expect(vertex.lat).to.be.a("number");
      expect(grid.toScheme(cell, grid.scheme)).to.equal(cell);
    }
  });
});

describe("Grid.toScheme", () => {
  test("nested and ring convert at the grid's level", () => {
    // at level 1, nested 2 == ring 4 (see the bitCombine parity test)
    const nested = new Grid({ scheme: "nested", level: 1 });
    const ring = new Grid({ scheme: "ring", level: 1 });

    expect(nested.toScheme(2n, "ring")).to.equal(4n);
    expect(ring.toScheme(4n, "nested")).to.equal(2n);

    // same-scheme conversions are the identity
    expect(nested.toScheme(2n, "nested")).to.equal(2n);
    expect(ring.toScheme(4n, "ring")).to.equal(4n);
  });

  test("converting to zuniq encodes the grid's level", () => {
    const nested = new Grid({ scheme: "nested", level: 4 });
    const zuniq = new Grid({ scheme: "zuniq", level: 4 });

    const id = nested.toScheme(164n, "zuniq");
    // the zuniq encoding of nested hash 164 at level 4: the hash sits above
    // a sentinel bit at position 2 * (29 - level)
    expect(id).to.equal(((164n << 1n) | 1n) << (2n * (29n - 4n)));
    expect(zuniq.toScheme(id, "nested")).to.equal(164n);
  });

  test("converting from zuniq uses the level encoded in the id", () => {
    // a level-0 id in a level-4 grid converts at level 0, mirroring how
    // `vertex` treats zuniq ids
    const zuniq = new Grid({ scheme: "zuniq", level: 4 });
    const coarse = healpixGeo.zuniq.bitCombine(0, 0, 0);

    expect(zuniq.toScheme(coarse, "nested")).to.equal(
      healpixGeo.nested.bitCombine(0, 0, 0),
    );
    // zuniq -> zuniq keeps the id as-is
    expect(zuniq.toScheme(coarse, "zuniq")).to.equal(coarse);
  });

  test("the level parameter overrides the level for zuniq targets", () => {
    const nested = new Grid({ scheme: "nested", level: 4 });

    // the grid's own level is a valid override
    expect(nested.toScheme(164n, "zuniq", 4)).to.equal(
      nested.toScheme(164n, "zuniq"),
    );
    // a coarser override reads (and encodes) the id at that level: the hash
    // sits above a sentinel bit at position 2 * (29 - level)
    expect(nested.toScheme(5n, "zuniq", 0)).to.equal(
      ((5n << 1n) | 1n) << (2n * 29n),
    );
    // ring ids are converted at the override level too: ring 4 == nested 2
    // at level 1
    const ring = new Grid({ scheme: "ring", level: 4 });
    const one = new Grid({ scheme: "nested", level: 1 });
    expect(ring.toScheme(4n, "zuniq", 1)).to.equal(one.toScheme(2n, "zuniq"));
    // and the cell id is validated against the override level
    expect(() => nested.toScheme(164n, "zuniq", 0)).to.throw(/out of range/);
  });

  test("the level parameter is rejected where it cannot apply", () => {
    const nested = new Grid({ scheme: "nested", level: 4 });

    // schemes whose ids do not encode the level reject the override
    expect(() => nested.toScheme(164n, "ring", 4)).to.throw(/only valid/);
    expect(() => nested.toScheme(164n, "nested", 4)).to.throw(/only valid/);

    // zuniq ids already carry their level, even for the identity
    const zuniq = new Grid({ scheme: "zuniq", level: 4 });
    const id = zuniq.bitCombine(3, 5);
    expect(() => zuniq.toScheme(id, "zuniq", 4)).to.throw(/already encode/);
    expect(() => zuniq.toScheme(id, "nested", 4)).to.throw(/already encode/);

    // and the level itself is validated
    expect(() => nested.toScheme(164n, "zuniq", 30)).to.throw(
      /`level` must be in \[0, 29\]/,
    );
    expect(() => nested.toScheme(164n, "zuniq", 2.5)).to.throw(/integer/);
    expect(() => nested.toScheme(164n, "zuniq", NaN)).to.throw(/integer/);
    // the level bound, not the u32 bound the narrowing helper would report
    expect(() => nested.toScheme(164n, "zuniq", -1)).to.throw(
      /`level` must be in \[0, 29\], got -1/,
    );
  });

  test("rejects invalid ids and unknown schemes", () => {
    const nested = new Grid({ scheme: "nested", level: 4 });
    expect(() => nested.toScheme(3072n, "ring")).to.throw(/out of range/);

    const zuniq = new Grid({ scheme: "zuniq", level: 4 });
    expect(() => zuniq.toScheme(0n, "nested")).to.throw(/zuniq/);

    // @ts-expect-error the target scheme must be one of the three names
    expect(() => nested.toScheme(0n, "bogus")).to.throw(/scheme/);
  });
});

describe("Grid ellipsoid state reuse", () => {
  test("repeated calls do not need re-parsing", () => {
    const grid = new Grid({
      scheme: "nested",
      level: 4,
      ellipsoid: wgs84,
    });

    const first = grid.vertex(164n, 0.5, 0.5);
    const second = grid.vertex(164n, 0.5, 0.5);
    expect(first.lon).to.equal(second.lon);
    expect(first.lat).to.equal(second.lat);
  });

  test("cells above 2^53 stay exact", () => {
    // level 29 nested ids exceed Number.MAX_SAFE_INTEGER
    const grid = new Grid({ scheme: "nested", level: 29 });
    // 12 * 4^29 cells, so this is the very last one (3458764513820540927)
    const cell = 12n * 4n ** 29n - 1n;
    expect(cell > BigInt(Number.MAX_SAFE_INTEGER)).to.equal(true);
    expect(() => grid.vertex(cell + 1n, 0.5, 0.5)).to.throw(/out of range/);

    const vertex = grid.vertex(cell, 0.5, 0.5);
    expect(vertex.lon).to.be.a("number");
    // and the id survives a scheme roundtrip bit-exactly
    const zuniq = new Grid({ scheme: "zuniq", level: 29 });
    expect(zuniq.toScheme(grid.toScheme(cell, "zuniq"), "nested")).to.equal(
      cell,
    );
  });
});
