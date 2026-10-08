import * as healpixGeo from "../pkg/healpix_geo.js";
import { Grid } from "../pkg/healpix_geo.js";
import { describe, expect, test } from "vitest";

// golden words derived from the packed-word layout documented in mortie-core
// ([4-bit prefix][27×2-bit body][6-bit suffix]): from_nested(nested, level)
const BASE0_L0 = 1152921504606846976n; // from_nested(0, 0)
const CELL164_L3 = 4107282860161892355n; // from_nested(164, 3)
const CELL41_L2 = 4107282860161892354n; // from_nested(41, 2), the parent of 164@3
const CELL10_L1 = 4035225266123964417n; // from_nested(10, 1), the parent of 41@2
// the remaining three children of 41@2, and the level-2 cell after it
const CELL165_L3 = 4125297258671374339n; // from_nested(165, 3)
const CELL166_L3 = 4143311657180856323n; // from_nested(166, 3)
const CELL167_L3 = 4161326055690338307n; // from_nested(167, 3)
const CELL42_L2 = 4179340454199820290n; // from_nested(42, 2)
// a max-encoded point word and the level-29 area cell covering it:
// from_nested_point(164 << 52) and from_nested(164 << 52, 29)
const POINT_L29 = 4107282860161892400n;
const AREA_L29 = 4107282860161892381n;

describe("morton statics", () => {
  test("level reads the embedded refinement level", () => {
    expect(healpixGeo.morton.level(BASE0_L0)).to.equal(0);
    expect(healpixGeo.morton.level(CELL164_L3)).to.equal(3);
  });

  test("level rejects invalid words", () => {
    expect(() => healpixGeo.morton.level(0n)).to.throw(
      "not a valid morton cell id",
    );
    expect(() => healpixGeo.morton.level(2n ** 64n - 1n)).to.throw();
  });

  test("ancestor truncates to the requested level", () => {
    expect(healpixGeo.morton.ancestor(CELL164_L3, 2)).to.equal(CELL41_L2);
    expect(healpixGeo.morton.ancestor(CELL164_L3, 1)).to.equal(CELL10_L1);
    // at or above the embedded level, the id is unchanged
    expect(healpixGeo.morton.ancestor(CELL164_L3, 3)).to.equal(CELL164_L3);
    expect(healpixGeo.morton.ancestor(CELL164_L3, 29)).to.equal(CELL164_L3);
  });

  test("ancestor validates its inputs", () => {
    expect(() => healpixGeo.morton.ancestor(0n, 2)).to.throw();
    expect(() => healpixGeo.morton.ancestor(CELL164_L3, 2.5)).to.throw();
    expect(() => healpixGeo.morton.ancestor(CELL164_L3, 30)).to.throw();
    expect(() => healpixGeo.morton.ancestor(CELL164_L3, -1)).to.throw();
  });

  test("contains is containment-is-truncation", () => {
    expect(healpixGeo.morton.contains(CELL41_L2, CELL164_L3)).to.equal(true);
    expect(healpixGeo.morton.contains(CELL10_L1, CELL164_L3)).to.equal(true);
    // not symmetric; reflexive
    expect(healpixGeo.morton.contains(CELL164_L3, CELL41_L2)).to.equal(false);
    expect(healpixGeo.morton.contains(CELL164_L3, CELL164_L3)).to.equal(true);
    // a different level-2 cell does not contain it
    expect(healpixGeo.morton.contains(BASE0_L0, CELL164_L3)).to.equal(false);
  });

  test("contains validates its inputs", () => {
    expect(() => healpixGeo.morton.contains(0n, CELL164_L3)).to.throw();
    expect(() =>
      healpixGeo.morton.contains(CELL164_L3, 2n ** 64n - 1n),
    ).to.throw();
  });

  test("a point word coarsens to the cell containing it", () => {
    expect(healpixGeo.morton.level(POINT_L29)).to.equal(29);
    // below level 29 a point and its level-29 area cell share every ancestor
    for (const level of [0, 3, 24, 28]) {
      expect(healpixGeo.morton.ancestor(POINT_L29, level)).to.equal(
        healpixGeo.morton.ancestor(AREA_L29, level),
      );
    }
    // at its own level (and above) the point is returned unchanged
    expect(healpixGeo.morton.ancestor(POINT_L29, 29)).to.equal(POINT_L29);
  });

  test("contains covers point words", () => {
    expect(healpixGeo.morton.contains(AREA_L29, POINT_L29)).to.equal(true);
    expect(healpixGeo.morton.contains(CELL164_L3, POINT_L29)).to.equal(true);
    expect(healpixGeo.morton.contains(CELL10_L1, POINT_L29)).to.equal(true);
    expect(healpixGeo.morton.contains(CELL165_L3, POINT_L29)).to.equal(false);
    // a point claims no area, so it contains nothing — `false`, not a throw
    expect(healpixGeo.morton.contains(POINT_L29, POINT_L29)).to.equal(false);
    expect(healpixGeo.morton.contains(POINT_L29, AREA_L29)).to.equal(false);
  });

  test("isPoint separates the kinds", () => {
    expect(healpixGeo.morton.isPoint(POINT_L29)).to.equal(true);
    expect(healpixGeo.morton.isPoint(AREA_L29)).to.equal(false);
    expect(healpixGeo.morton.isPoint(CELL164_L3)).to.equal(false);
    expect(() => healpixGeo.morton.isPoint(0n)).to.throw(
      "not a valid morton cell id",
    );
  });
});

describe("morton grid", () => {
  const grid = new Grid({ scheme: "morton", level: 3 });
  const nested = new Grid({ scheme: "nested", level: 3 });

  test("coordinates match the nested scheme", () => {
    const expected = nested.healpixToLonLat(new BigUint64Array([164n]));
    const actual = grid.healpixToLonLat(new BigUint64Array([CELL164_L3]));
    expect(Array.from(actual)).to.deep.equal(Array.from(expected));
  });

  test("lonLatToHealpix encodes at the grid's level", () => {
    const centers = grid.healpixToLonLat(new BigUint64Array([CELL164_L3]));
    const roundtrip = grid.lonLatToHealpix(centers);
    expect(Array.from(roundtrip)).to.deep.equal([CELL164_L3]);
  });

  test("ids are read at their embedded level", () => {
    // a level-0 id in a level-3 grid: the base-cell vertex, as in the zuniq
    // scheme
    const vertex = grid.vertex(BASE0_L0, 0.0, 0.0);
    expect(vertex.lon).to.be.closeTo(45.0, 1e-4);
    expect(vertex.lat).to.be.closeTo(0.0, 1e-4);
  });

  test("invalid and non-canonical ids throw instead of trapping", () => {
    expect(() => grid.vertex(0n, 0.5, 0.5)).to.throw(
      "not a valid morton cell id",
    );
    expect(() => grid.vertex(2n ** 64n - 1n, 0.5, 0.5)).to.throw();
    // junk below the encoded level makes the word non-canonical
    expect(() => grid.vertex(CELL164_L3 | (1n << 30n), 0.5, 0.5)).to.throw();
    // a max-encoded point word claims no area, so it is not a cell id
    expect(() => grid.vertex(POINT_L29, 0.5, 0.5)).to.throw(
      "not a morton cell id",
    );
    expect(() => grid.toScheme(POINT_L29, "nested")).to.throw();
  });

  test("toScheme round-trips through nested and zuniq", () => {
    expect(grid.toScheme(CELL164_L3, "nested")).to.equal(164n);
    expect(nested.toScheme(164n, "morton")).to.equal(CELL164_L3);

    const zuniq = grid.toScheme(CELL164_L3, "zuniq");
    const zuniqGrid = new Grid({ scheme: "zuniq", level: 3 });
    expect(zuniqGrid.toScheme(zuniq, "morton")).to.equal(CELL164_L3);

    // identity, as for zuniq
    expect(grid.toScheme(CELL164_L3, "morton")).to.equal(CELL164_L3);
  });

  test("toScheme round-trips a level-29 id with a nonzero tail", () => {
    // the packed suffix folds the two deepest tuples into one number, so an
    // id whose deepest tuples are zero leaves that arithmetic untested
    const deep = new Grid({ scheme: "nested", level: 29 });
    const deepMorton = new Grid({ scheme: "morton", level: 29 });
    const hash = (2n << 58n) | 0xc9b7n; // first cell of base 2, nonzero tail

    const word = deep.toScheme(hash, "morton");
    expect(healpixGeo.morton.level(word)).to.equal(29);
    expect(deepMorton.toScheme(word, "nested")).to.equal(hash);
  });

  test("toScheme accepts a level override only from level-free schemes", () => {
    expect(nested.toScheme(164n, "morton", 3)).to.equal(CELL164_L3);
    expect(() => grid.toScheme(CELL164_L3, "morton", 3)).to.throw();
    expect(() => nested.toScheme(164n, "ring", 3)).to.throw();
  });

  test("a raw unsigned sort is a preorder traversal", () => {
    // the property gridlook's hive addressing relies on: sorting mixed-level
    // *area* ids as unsigned integers walks the tree in preorder, so every
    // parent lands immediately before its first child and its subtree is an
    // unbroken run. (Point words sort after the entire level-28/29 area
    // region of their body, so they are not part of any subtree run.) 41@2
    // with all four of its children, plus two cells outside the subtree,
    // deliberately shuffled.
    const shuffled = [
      CELL167_L3,
      CELL42_L2,
      CELL164_L3,
      CELL10_L1,
      CELL166_L3,
      CELL41_L2,
      CELL165_L3,
    ];
    // BigInt needs the explicit comparator; the default sort is lexicographic
    const sorted = [...shuffled].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));

    expect(sorted).to.deep.equal([
      CELL10_L1,
      CELL41_L2,
      CELL164_L3,
      CELL165_L3,
      CELL166_L3,
      CELL167_L3,
      CELL42_L2,
    ]);

    // immediately: the first child is the very next word, not merely a later
    // one
    const parent = sorted.indexOf(CELL41_L2);
    expect(sorted[parent + 1]).to.equal(CELL164_L3);
    expect(CELL41_L2 + 1n).to.equal(CELL164_L3);

    // contiguous: the run after the parent is exactly its subtree, with
    // nothing foreign interleaved
    const subtree = sorted.slice(parent, parent + 5);
    expect(
      subtree.every((id) => healpixGeo.morton.contains(CELL41_L2, id)),
    ).to.equal(true);
    expect(healpixGeo.morton.contains(CELL41_L2, sorted[parent - 1])).to.equal(
      false,
    );
    expect(healpixGeo.morton.contains(CELL41_L2, sorted[parent + 5])).to.equal(
      false,
    );
  });
});
