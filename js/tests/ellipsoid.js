import init, * as healpixGeo from "../pkg/index.js";

beforeAll(async () => {
  await init();
});

describe("nested vertex", () => {
  test("default ellipsoid", () => {
    expect(healpixGeo.vertexNested(4, 0, 1.0, 1.0, null)).toBe({
      lon: 31.0,
      lat: 31.0,
    });
  });
});
