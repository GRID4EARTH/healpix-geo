import init, * as healpixGeo from "../pkg/index.js";
import { Grid } from "../pkg/index.js";
import { describe, expect, test } from "vitest";

const wgs84 = {
  semi_major_axis: 6378137.0,
  inverse_flattening: 298.257223563,
};

describe("Grid.vertices", () => {
  test("matches the scalar vertex loop over the full gridlook geometry", () => {
    // 12 base cells x 65 x 65 vertices, sphere and WGS84 — the shape
    // gridlook's makeHealpixGeometry builds, checked for bit equality
    const steps = 65;
    const scale = 1 / (steps - 1);

    let compared = 0;
    const mismatches: string[] = [];

    for (const ellipsoid of [null, wgs84]) {
      const grid = new Grid({
        scheme: "nested",
        level: 0,
        ellipsoid,
      });

      for (let cell = 0n; cell < 12n; ++cell) {
        const bulk = grid.vertices(cell, steps);
        expect(bulk.length).to.equal(steps * steps * 2);

        let k = 0;
        for (let i = 0; i < steps; ++i) {
          for (let j = 0; j < steps; ++j) {
            const scalar = grid.vertex(cell, i * scale, j * scale);
            if (bulk[k] !== scalar.lon || bulk[k + 1] !== scalar.lat) {
              mismatches.push(`cell ${cell} (${i}, ${j})`);
            }
            k += 2;
            compared += 1;
          }
        }
      }
    }

    expect(mismatches).to.deep.equal([]);
    expect(compared).to.equal(2 * 12 * steps * steps);
  });

  test("matches the scalar vertex loop", () => {
    const steps = 5;
    for (const ellipsoid of [null, wgs84]) {
      const grid = new Grid({
        scheme: "nested",
        level: 4,
        ellipsoid,
      });

      const bulk = grid.vertices(164n, steps);
      expect(bulk).to.be.instanceOf(Float64Array);
      expect(bulk.length).to.equal(steps * steps * 2);

      let k = 0;
      for (let i = 0; i < steps; ++i) {
        const u = i / (steps - 1);
        for (let j = 0; j < steps; ++j) {
          const v = j / (steps - 1);
          const scalar = grid.vertex(164n, u, v);
          expect(bulk[k++]).to.equal(scalar.lon);
          expect(bulk[k++]).to.equal(scalar.lat);
        }
      }
    }
  });

  test("works for zuniq cells (level from the id)", () => {
    const grid = new Grid({ scheme: "zuniq", level: 4 });
    const cell = healpixGeo.zuniq.bitCombine(4, 3, 5);

    const bulk = grid.vertices(cell, 3);
    const corner = grid.vertex(cell, 0, 0);
    expect(bulk[0]).to.equal(corner.lon);
    expect(bulk[1]).to.equal(corner.lat);
  });

  test("rejects steps < 2", () => {
    const grid = new Grid({ scheme: "nested", level: 4 });
    expect(() => grid.vertices(164n, 1)).to.throw(/at least 2/);
  });

  test("rejects step counts wasm-bindgen would have coerced", () => {
    // declared as `u32`, `2**32 + 2` arrived as `steps = 2`
    const grid = new Grid({ scheme: "nested", level: 4 });

    expect(() => grid.vertices(164n, 2 ** 32 + 2)).to.throw(/must be in/);
    expect(() => grid.vertices(164n, 2.5)).to.throw(/integer/);
    expect(() => grid.vertices(164n, NaN)).to.throw(/integer/);
  });

  test("cells above 2^53 stay exact", () => {
    const grid = new Grid({ scheme: "nested", level: 29 });
    // 12 * 4^29 cells, so this is the very last one (3458764513820540927)
    const cell = 12n * 4n ** 29n - 1n;
    expect(cell > BigInt(Number.MAX_SAFE_INTEGER)).to.equal(true);
    expect(() => grid.vertices(cell + 1n, 2)).to.throw(/out of range/);

    const bulk = grid.vertices(cell, 2);
    const corner = grid.vertex(cell, 0, 0);
    expect(bulk[0]).to.equal(corner.lon);
    expect(bulk[1]).to.equal(corner.lat);
  });
});

describe("Grid.healpixToLonLat and Grid.lonLatToHealpix", () => {
  test("roundtrip through typed arrays", () => {
    const grid = new Grid({
      scheme: "ring",
      level: 4,
      ellipsoid: wgs84,
    });
    const cells = new BigUint64Array([0n, 164n, 700n]);

    const centers = grid.healpixToLonLat(cells);
    expect(centers).to.be.instanceOf(Float64Array);
    expect(centers.length).to.equal(cells.length * 2);

    const roundtrip = grid.lonLatToHealpix(centers);
    expect(roundtrip).to.be.instanceOf(BigUint64Array);
    expect([...roundtrip]).to.deep.equal([...cells]);
  });

  test("healpixToLonLat matches the scalar statics", () => {
    const sphere = healpixGeo.Ellipsoid.from(null);

    const nested = new Grid({ scheme: "nested", level: 4 });
    const centers = nested.healpixToLonLat(new BigUint64Array([0n, 164n]));
    const staticCenter = healpixGeo.nested.healpixToLonLat(164n, 4, sphere);
    expect(centers[2]).to.equal(staticCenter.lon);
    expect(centers[3]).to.equal(staticCenter.lat);

    // the zuniq statics take no level for centers; the grid object exposes
    // the exact same call shape for all three schemes
    const zuniq = new Grid({ scheme: "zuniq", level: 0 });
    const cell = healpixGeo.zuniq.bitCombine(0, 0, 0);
    const center = zuniq.healpixToLonLat(new BigUint64Array([cell]));
    const staticZuniq = healpixGeo.zuniq.healpixToLonLat(cell, sphere);
    expect(center[0]).to.equal(staticZuniq.lon);
    expect(center[1]).to.equal(staticZuniq.lat);
  });

  test("lonLatToHealpix matches the scalar static", () => {
    const grid = new Grid({ scheme: "nested", level: 4 });
    const sphere = healpixGeo.Ellipsoid.from(null);

    const cells = grid.lonLatToHealpix(
      new Float64Array([16.875, 38.68, 45.0, 0.0]),
    );
    expect(cells[0]).to.equal(
      healpixGeo.nested.lonLatToHealpix(16.875, 38.68, 4, sphere),
    );
    expect(cells[1]).to.equal(
      healpixGeo.nested.lonLatToHealpix(45.0, 0.0, 4, sphere),
    );
  });

  test("lonLatToHealpix rejects odd-length input", () => {
    const grid = new Grid({ scheme: "nested", level: 4 });
    expect(() => grid.lonLatToHealpix(new Float64Array([1, 2, 3]))).to.throw(
      /even length/,
    );
  });

  test("lonLatToHealpix rejects an invalid coordinate and names its index", () => {
    // a single NaN in a long buffer used to trap the whole wasm instance:
    // no message, no index, unlike every other bulk failure here
    const grid = new Grid({ scheme: "nested", level: 4 });

    expect(() =>
      grid.lonLatToHealpix(new Float64Array([45, 0, 45, 0, 0, 100])),
    ).to.throw(/lonlats\[2\]/);
    expect(() => grid.lonLatToHealpix(new Float64Array([45, NaN]))).to.throw(
      /lonlats\[0\]/,
    );
    expect(() => grid.lonLatToHealpix(new Float64Array([NaN, 0]))).to.throw(
      /lonlats\[0\]/,
    );

    // the poles themselves are legal, and so is an unwrapped longitude
    expect(grid.lonLatToHealpix(new Float64Array([0, 90])).length).to.equal(1);
    expect(
      grid.lonLatToHealpix(new Float64Array([-720.5, 45])).length,
    ).to.equal(1);

    // and the instance is still alive afterwards
    expect(grid.lonLatToHealpix(new Float64Array([45, 0])).length).to.equal(1);
  });

  test("for zuniq, the lonlat methods are not inverses across levels", () => {
    // `healpixToLonLat` reads each id at its embedded level;
    // `lonLatToHealpix` always encodes at the grid's level, so a mixed-level
    // batch comes back all-fine
    const grid = new Grid({ scheme: "zuniq", level: 4 });
    const coarse = healpixGeo.zuniq.bitCombine(0, 0, 0);
    const fine = healpixGeo.zuniq.bitCombine(4, 3, 5);

    const roundtrip = grid.lonLatToHealpix(
      grid.healpixToLonLat(new BigUint64Array([coarse, fine])),
    );

    expect(roundtrip[1]).to.equal(fine);
    expect(roundtrip[0]).to.not.equal(coarse);
    // the level-4 cell containing the level-0 center
    const center = grid.healpixToLonLat(new BigUint64Array([coarse]));
    expect(roundtrip[0]).to.equal(grid.lonLatToHealpix(center)[0]);
  });

  test("healpixToLonLat rejects an invalid cell and names its index", () => {
    // one bad element in a batch used to trap the whole wasm instance
    const grid = new Grid({ scheme: "nested", level: 4 });
    expect(() =>
      grid.healpixToLonLat(new BigUint64Array([0n, 164n, 3072n])),
    ).to.throw(/cells\[2\]/);

    const zuniq = new Grid({ scheme: "zuniq", level: 4 });
    expect(() => zuniq.healpixToLonLat(new BigUint64Array([0n]))).to.throw(
      /zuniq/,
    );

    // and the instance is still alive afterwards
    expect(grid.healpixToLonLat(new BigUint64Array([164n])).length).to.equal(2);
  });
});

describe("Grid.bitCombineTable", () => {
  test("matches the scalar bitCombine (gridlook unshuffle layout)", () => {
    const size = 8;
    const grid = new Grid({ scheme: "nested", level: 3 }); // nside == 8

    const table = grid.bitCombineTable(size);
    expect(table).to.be.instanceOf(BigUint64Array);
    expect(table.length).to.equal(size * size);

    // gridlook's loop: for i { for j { temp[idx++] = bitCombine(j, i) } }
    let idx = 0;
    for (let i = 0; i < size; ++i) {
      for (let j = 0; j < size; ++j) {
        expect(table[idx++]).to.equal(healpixGeo.nested.bitCombine(3, j, i));
      }
    }
  });

  test("matches the scalar bitCombine for size < nside", () => {
    // the table takes its z-order curve from `size`, `bitCombine` from the
    // grid's level — at `size == nside` the two coincide trivially, so the
    // equivalence is only really tested below it
    for (const scheme of ["nested", "ring", "zuniq"] as const) {
      const size = 4;
      const grid = new Grid({ scheme, level: 5 }); // nside == 32

      const table = grid.bitCombineTable(size);
      expect(table.length).to.equal(size * size);

      for (let row = 0; row < size; ++row) {
        for (let col = 0; col < size; ++col) {
          expect(table[row * size + col]).to.equal(grid.bitCombine(col, row));
        }
      }
    }
  });

  test("rejects sizes wasm-bindgen would have coerced", () => {
    // declared as `u32`, `2**32 + 2` arrived as `size = 2`
    const grid = new Grid({ scheme: "nested", level: 8 });

    expect(() => grid.bitCombineTable(2 ** 32 + 2)).to.throw(/must be in/);
    expect(() => grid.bitCombineTable(2.5)).to.throw(/integer/);
    expect(() => grid.bitCombineTable(-4)).to.throw(/must be in/);
  });

  test("does not depend on the grid's level", () => {
    const size = 8;
    const shallow = new Grid({ scheme: "nested", level: 3 }); // nside == 8
    const deep = new Grid({ scheme: "nested", level: 12 });

    expect([...deep.bitCombineTable(size)]).to.deep.equal([
      ...shallow.bitCombineTable(size),
    ]);
  });

  test("rejects sizes the grid cannot represent", () => {
    // each of these used to return silently wrong values, and the ring case
    // trapped the wasm instance
    expect(() =>
      new Grid({ scheme: "nested", level: 0 }).bitCombineTable(4),
    ).to.throw(/at most/);
    expect(() =>
      new Grid({ scheme: "nested", level: 3 }).bitCombineTable(16),
    ).to.throw(/at most/);
    expect(() =>
      new Grid({ scheme: "nested", level: 8 }).bitCombineTable(300),
    ).to.throw(/power of two/);
    expect(() =>
      new Grid({ scheme: "ring", level: 1 }).bitCombineTable(256),
    ).to.throw(/at most/);

    // the module is still usable, and the legal call still works
    const grid = new Grid({ scheme: "ring", level: 1 });
    expect(grid.bitCombineTable(2).length).to.equal(4);
  });
});
