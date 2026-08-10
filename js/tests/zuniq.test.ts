import init, * as healpixGeo from "../pkg/index.js";
import { Coordinate, Ellipsoid } from "../pkg/index.js";
import { describe, expect, test } from "vitest";

describe("zuniq bitcombine", () => {
  test("level 0 south", () => {
    const depth: number = 3;
    const i: number = 0.0;
    const j: number = 0.0;

    expect(healpixGeo.zuniq.bitCombine(depth, i, j)).to.equal(
      4503599627370496n,
    );
  });
});

describe("zuniq healpixToLonLat", () => {
  test("default-ellipsoid", () => {
    const actual: Coordinate = healpixGeo.zuniq.healpixToLonLat(
      164n,
      Ellipsoid.from(null),
    );
    expect(actual).to.have.a.property("lon", 45.00000100582838);
    expect(actual).to.have.a.property("lat", 9.960692539601928e-7);
  });

  test("sphere", () => {
    const sphere: Ellipsoid = Ellipsoid.from({ radius: 6371000 });
    const actual: Coordinate = healpixGeo.zuniq.healpixToLonLat(164n, sphere);
    expect(actual).to.have.a.property("lon", 45.00000100582838);
    expect(actual).to.have.a.property("lat", 9.960692539601928e-7);
  });
  test("ellipsoid", () => {
    const ellipsoid: Ellipsoid = Ellipsoid.from({
      semi_major_axis: 6378137.0,
      inverse_flattening: 298.257223563,
    });
    const actual: Coordinate = healpixGeo.zuniq.healpixToLonLat(
      164n,
      ellipsoid,
    );
    expect(actual).to.have.a.property("lon", 45.00000100582838);
    expect(actual).to.have.a.property("lat", 0.000001000541586366739);
  });
});

describe("zuniq vertex", () => {
  test("default ellipsoid northern", () => {
    const cellId: bigint = 4n;
    const ellipsoid: Ellipsoid = Ellipsoid.from(null);
    const actual: Coordinate = healpixGeo.zuniq.vertex(
      cellId,
      1.0,
      1.0,
      ellipsoid,
    );
    expect(actual.lon).to.equal(45);
    expect(actual.lat).to.equal(2.845912154171979e-7);
  });

  test("sphere eastern", () => {
    const cellId: bigint = 4n;
    const ellipsoid: Ellipsoid = Ellipsoid.from({
      radius: 6371000,
    });

    const actual: Coordinate = healpixGeo.zuniq.vertex(
      cellId,
      1.0,
      0,
      ellipsoid,
    );
    expect(actual).to.have.a.property("lon", 45.00000016763806);
    expect(actual).to.have.a.property("lat", 1.4229560770859896e-7);
  });
  test("ellipsoid western", () => {
    const cellId: bigint = 4n;
    const ellipsoid: Ellipsoid = Ellipsoid.from({
      semi_major_axis: 6378137.0,
      inverse_flattening: 298.257223563,
    });

    const actual: Coordinate = healpixGeo.zuniq.vertex(
      cellId,
      0,
      1.0,
      ellipsoid,
    );
    expect(actual).to.have.a.property("lon", 44.99999983236194);
    expect(actual).to.have.a.property("lat", 1.429345123381056e-7);
  });

  test("rejects offsets outside [0, 1]", () => {
    const ellipsoid: Ellipsoid = Ellipsoid.from(null);
    const cellId: bigint = 288230376151711744n;
    expect(() => healpixGeo.zuniq.vertex(cellId, 2, 0, ellipsoid)).to.throw(
      /\[0, 1\]/,
    );
  });
});
