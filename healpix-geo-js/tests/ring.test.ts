import init, * as healpixGeo from "../pkg/index.js";
import { Coordinate, Ellipsoid } from "../pkg/index.js";
import { describe, expect, test } from "vitest";

describe("ring bitcombine", () => {
  test("level 0 south", () => {
    const depth: number = 3;
    const i: number = 0.0;
    const j: number = 0.0;

    expect(healpixGeo.ring.bitCombine(depth, i, j)).to.equal(340n);
  });
});

describe("ring healpixToLonLat", () => {
  test("default-ellipsoid", () => {
    const actual: Coordinate = healpixGeo.ring.healpixToLonLat(
      164n,
      4,
      Ellipsoid.from(null),
    );
    expect(actual).to.have.a.property("lon", 205);
    expect(actual).to.have.a.property("lat", 63.44828368030105);
  });

  test("sphere", () => {
    const sphere: Ellipsoid = Ellipsoid.from({ radius: 6371000 });
    const actual: Coordinate = healpixGeo.ring.healpixToLonLat(164n, 4, sphere);
    expect(actual).to.have.a.property("lon", 205);
    expect(actual).to.have.a.property("lat", 63.44828368030105);
  });
  test("ellipsoid", () => {
    const ellipsoid: Ellipsoid = Ellipsoid.from({
      semi_major_axis: 6378137.0,
      inverse_flattening: 298.257223563,
    });
    const actual: Coordinate = healpixGeo.ring.healpixToLonLat(
      164n,
      4,
      ellipsoid,
    );
    expect(actual).to.have.a.property("lon", 205);
    expect(actual).to.have.a.property("lat", 63.55072709071251);
  });
});

describe("ring vertex", () => {
  test("default ellipsoid northern", () => {
    const cellId: bigint = 4n;
    const ellipsoid: Ellipsoid = Ellipsoid.from(null);
    const actual: Coordinate = healpixGeo.ring.vertex(
      cellId,
      0,
      1.0,
      1.0,
      ellipsoid,
    );
    expect(actual.lon).to.equal(0);
    expect(actual.lat).to.equal(41.810314895778596);
  });

  test("sphere eastern", () => {
    const cellId: bigint = 4n;
    const depth: number = 0;
    const ellipsoid: Ellipsoid = Ellipsoid.from({
      radius: 6371000,
    });

    const actual: Coordinate = healpixGeo.ring.vertex(
      cellId,
      depth,
      1.0,
      0,
      ellipsoid,
    );
    expect(actual).to.have.a.property("lon", 45);
    expect(actual).to.have.a.property("lat", 0);
  });
  test("ellipsoid western", () => {
    const cellId: bigint = 4n;
    const depth: number = 0;
    const ellipsoid: Ellipsoid = Ellipsoid.from({
      semi_major_axis: 6378137.0,
      inverse_flattening: 298.257223563,
    });

    const actual: Coordinate = healpixGeo.ring.vertex(
      cellId,
      depth,
      0,
      1.0,
      ellipsoid,
    );
    expect(actual).to.have.a.property("lon", 315);
    expect(actual).to.have.a.property("lat", 0);
  });

  test("rejects offsets outside [0, 1]", () => {
    const ellipsoid: Ellipsoid = Ellipsoid.from(null);
    expect(() => healpixGeo.ring.vertex(4n, 0, -0.1, 0, ellipsoid)).to.throw(
      /\[0, 1\]/,
    );
  });
});
