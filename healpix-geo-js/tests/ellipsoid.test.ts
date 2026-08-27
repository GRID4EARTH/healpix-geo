import init, * as healpixGeo from "../pkg/index.js";
import { Ellipsoid } from "../pkg/index.js";
import { describe, expect, test } from "vitest";

describe("Ellipsoid.from", () => {
  test("null gives the default sphere", () => {
    const result = Ellipsoid.from(null);
    expect(result.isSphere).to.equal(true);
    expect(result.semiMajorAxis).to.equal(6370997.0);
    expect(result.flattening).to.equal(0);
  });

  test("undefined gives the default sphere", () => {
    const explicit = Ellipsoid.from(undefined);
    expect(explicit.isSphere).to.equal(true);
    expect(explicit.semiMajorAxis).to.equal(6370997.0);

    // a missing argument behaves the same way
    const missing = (Ellipsoid.from as () => Ellipsoid)();
    expect(missing.isSphere).to.equal(true);
  });

  test("sphere", () => {
    const result = Ellipsoid.from({ name: "sphere", radius: 6371000.0 });
    expect(result.isSphere).to.equal(true);
    expect(result.semiMajorAxis).to.equal(6371000);
  });

  test("ellipsoid from semi-major axis and inverse flattening", () => {
    const result = Ellipsoid.from({
      name: "WGS84",
      semi_major_axis: 6378137.0,
      inverse_flattening: 298.257223563,
    });
    expect(result.isSphere).to.equal(false);
    expect(result.semiMajorAxis).to.equal(6378137.0);
    expect(result.flattening).to.be.closeTo(1 / 298.257223563, 1e-15);
  });

  test("ellipsoid from semi-major axis and semi-minor axis", () => {
    const result = Ellipsoid.from({
      name: "WGS84",
      semi_major_axis: 6378137.0,
      semi_minor_axis: 6356752.314245179,
    });
    expect(result.isSphere).to.equal(false);
    expect(result.semiMajorAxis).to.equal(6378137.0);
    expect(result.flattening).to.be.closeTo(1 / 298.257223563, 1e-9);
  });

  test("invalid input throws", () => {
    // @ts-expect-error the object matches none of the accepted shapes
    expect(() => Ellipsoid.from({ bogus: 1 })).to.throw();
  });

  test("documented extra keys type-check on a fresh object literal", () => {
    // the excess-property check only fires on a fresh literal, so this has to
    // stay inline: hoisting it into a `const` would stop testing the type
    const result = Ellipsoid.from({
      name: "WGS84",
      epsg: 7030,
      semi_major_axis: 6378137.0,
      inverse_flattening: 298.257223563,
    });
    expect(result.semiMajorAxis).to.equal(6378137.0);
  });

  test("rejects degenerate reference bodies", () => {
    expect(() => Ellipsoid.from({ radius: 0 })).to.throw(/positive/);
    expect(() => Ellipsoid.from({ radius: -1 })).to.throw(/positive/);
    expect(() => Ellipsoid.from({ radius: Infinity })).to.throw(/positive/);

    // `inverse_flattening: 0` reads like "a sphere" and used to give
    // `flattening: Infinity`
    expect(() =>
      Ellipsoid.from({ semi_major_axis: 6378137.0, inverse_flattening: 0 }),
    ).to.throw(/greater than 1/);
    expect(() =>
      Ellipsoid.from({ semi_major_axis: 0, inverse_flattening: 298.257223563 }),
    ).to.throw(/positive/);

    expect(() =>
      Ellipsoid.from({ semi_major_axis: 6378137.0, semi_minor_axis: 0 }),
    ).to.throw(/positive/);
    expect(() =>
      Ellipsoid.from({
        semi_major_axis: 6356752.314245179,
        semi_minor_axis: 6378137.0,
      }),
    ).to.throw(/must not exceed/);

    // and the module is still usable afterwards
    expect(Ellipsoid.from({ radius: 6371000.0 }).isSphere).to.equal(true);
  });
});

describe("handle reuse", () => {
  test("the same handle survives repeated calls", () => {
    const ellipsoid = Ellipsoid.from({
      semi_major_axis: 6378137.0,
      inverse_flattening: 298.257223563,
    });

    // the handle is borrowed, not consumed: repeated calls with the same
    // handle must keep working
    const first = healpixGeo.nested.healpixToLonLat(164n, 4, ellipsoid);
    const second = healpixGeo.nested.healpixToLonLat(164n, 4, ellipsoid);

    expect(first.lon).to.equal(second.lon);
    expect(first.lat).to.equal(second.lat);
    expect(ellipsoid.semiMajorAxis).to.equal(6378137.0);
  });
});
