import init, * as healpixGeo from "../pkg/index.js";
import { describe, expect, test } from "vitest";

describe("parseEllipsoid", () => {
  test("null", () => {
    const result = healpixGeo.parseEllipsoid(null);
    expect(result).to.have.property("radius", 6370997.0);
  });

  test("sphere", () => {
    const ellipsoid = { name: "sphere", radius: 6371000.0 };
    const result = healpixGeo.parseEllipsoid(ellipsoid);
    expect(result).to.have.property("radius", 6371000);
  });

  test("ellipsoid from semi-major axis and inverse flattening", () => {
    const ellipsoid = {
      name: "WGS84",
      semi_major_axis: 6378137.0,
      inverse_flattening: 298.257223563,
    };
    const result = healpixGeo.parseEllipsoid(ellipsoid);
    expect(result).to.have.property(
      "semi_major_axis",
      ellipsoid.semi_major_axis,
    );
    expect(result).to.have.property(
      "inverse_flattening",
      ellipsoid.inverse_flattening,
    );
  });
  test("ellipsoid from semi-major axis and semi-minor axis", () => {
    const ellipsoid = {
      name: "WGS84",
      semi_major_axis: 6378137.0,
      semi_minor_axis: 6356752.314245179,
    };
    const result = healpixGeo.parseEllipsoid(ellipsoid);
    expect(result).to.have.property(
      "semi_major_axis",
      ellipsoid.semi_major_axis,
    );
    expect(result).to.have.property(
      "semi_minor_axis",
      ellipsoid.semi_minor_axis,
    );
  });
});
