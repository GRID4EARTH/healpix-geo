import init, * as healpixGeo from "../pkg/index.js";
import { Coordinate, TEllipsoid } from "../pkg/index.js";
import { describe, expect, test } from "vitest";

describe("nested bitcombine", () => {
  test("level 0 south", () => {
    const depth: Number = 3;
    const i: Number = 0.0;
    const j: Number = 0.0;

    expect(healpixGeo.nested.bitCombine(depth, i, j)).to.equal(0n);
  });
});

describe("nested healpixToLonLat", () => {
  test("default-ellipsoid", () => {
    const actual: Coordinate = healpixGeo.nested.healpixToLonLat(164n, 4, null);
    expect(actual).to.have.a.property("lon", 16.875);
    expect(actual).to.have.a.property("lat", 38.68218745348944);
  });

  test("sphere", () => {
    const sphere: TEllipsoid = healpixGeo.parseEllipsoid({ radius: 6371000 });
    const actual: Coordinate = healpixGeo.nested.healpixToLonLat(
      164n,
      4,
      sphere,
    );
    expect(actual).to.have.a.property("lon", 16.875);
    expect(actual).to.have.a.property("lat", 38.68218745348944);
  });
  test("ellipsoid", () => {
    const ellipsoid: TEllipsoid = healpixGeo.parseEllipsoid({
      semi_major_axis: 6378137.0,
      inverse_flattening: 298.257223563,
    });
    const actual: Coordinate = healpixGeo.nested.healpixToLonLat(
      164n,
      4,
      ellipsoid,
    );
    expect(actual).to.have.a.property("lon", 16.875);
    expect(actual).to.have.a.property("lat", 38.807447731964224);
  });
});

describe("nested vertex", () => {
  test("default ellipsoid northern", () => {
    const cellId: bigint = 4n;
    const ellipsoid: TEllipsoid = null;
    const actual: Coordinate = healpixGeo.nested.vertex(
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
    const ellipsoid: TEllipsoid = healpixGeo.parseEllipsoid({
      radius: 6371000,
    });

    const actual: Coordinate = healpixGeo.nested.vertex(
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
    const ellipsoid: TEllipsoid = healpixGeo.parseEllipsoid({
      semi_major_axis: 6378137.0,
      inverse_flattening: 298.257223563,
    });

    const actual: Coordinate = healpixGeo.nested.vertex(
      cellId,
      depth,
      0,
      1.0,
      ellipsoid,
    );
    expect(actual).to.have.a.property("lon", 315);
    expect(actual).to.have.a.property("lat", 0);
  });
});
