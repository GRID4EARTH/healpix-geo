import type { EllipsoidInput, GridOptions } from "../pkg/index.js";
import { Ellipsoid, Grid } from "../pkg/index.js";
import { describe, expectTypeOf, test } from "vitest";

// The generated `.d.ts` is the API surface TypeScript consumers actually see.
// `JsValue` parameters default to `any` there, which silently drops the input
// contract, so the hand-written `typescript_custom_section` types are pinned
// here. These are type-level assertions: vitest's typecheck runs them through
// `tsc`, nothing here executes.

describe("generated index.d.ts", () => {
  test("EllipsoidInput accepts the documented shapes", () => {
    expectTypeOf<{ radius: number }>().toExtend<EllipsoidInput>();
    expectTypeOf<{
      semi_major_axis: number;
      inverse_flattening: number;
    }>().toExtend<EllipsoidInput>();
    expectTypeOf<{
      semi_major_axis: number;
      semi_minor_axis: number;
    }>().toExtend<EllipsoidInput>();

    // the documented "extra keys are ignored" behavior type-checks
    expectTypeOf<{
      name: string;
      epsg: number;
      radius: number;
    }>().toExtend<EllipsoidInput>();

    // an incomplete shape does not
    expectTypeOf<{ semi_major_axis: number }>().not.toExtend<EllipsoidInput>();
  });

  test("ellipsoid entry points are typed, not `any`", () => {
    // `toEqualTypeOf` distinguishes `any`, so this fails if the parameter
    // ever degrades back to the wasm-bindgen default
    expectTypeOf(Ellipsoid.from)
      .parameter(0)
      .toEqualTypeOf<EllipsoidInput | null | undefined>();
    expectTypeOf(Ellipsoid.from).returns.toEqualTypeOf<Ellipsoid>();
  });

  test("the Grid constructor takes typed options, not `any`", () => {
    expectTypeOf(Grid).constructorParameters.toEqualTypeOf<[GridOptions]>();
    expectTypeOf<GridOptions["scheme"]>().toEqualTypeOf<
      "nested" | "ring" | "zuniq"
    >();
    expectTypeOf<GridOptions["level"]>().toEqualTypeOf<number>();
    expectTypeOf<GridOptions["ellipsoid"]>().toEqualTypeOf<
      EllipsoidInput | null | undefined
    >();
  });

  test("the scheme getter is narrowed to the literal union", () => {
    // widened to `string`, `other.toScheme(cell, grid.scheme)` does not
    // type-check and `switch (grid.scheme)` gets no exhaustiveness check
    expectTypeOf<Grid["scheme"]>().toEqualTypeOf<"nested" | "ring" | "zuniq">();
    expectTypeOf<Grid["level"]>().toEqualTypeOf<number>();
  });

  test("toScheme narrows its target and takes an optional level", () => {
    expectTypeOf<Grid["toScheme"]>()
      .parameter(1)
      .toEqualTypeOf<"nested" | "ring" | "zuniq">();
    expectTypeOf<Grid["toScheme"]>()
      .parameter(2)
      .toEqualTypeOf<number | null | undefined>();
  });

  test("the bulk methods exchange typed arrays", () => {
    expectTypeOf<Grid["vertices"]>().toEqualTypeOf<
      (cell: bigint, steps: number) => Float64Array
    >();
    expectTypeOf<Grid["healpixToLonLat"]>().toEqualTypeOf<
      (cells: BigUint64Array) => Float64Array
    >();
    expectTypeOf<Grid["lonLatToHealpix"]>().toEqualTypeOf<
      (lonlats: Float64Array) => BigUint64Array
    >();
    expectTypeOf<Grid["bitCombineTable"]>().toEqualTypeOf<
      (size: number) => BigUint64Array
    >();
  });
});
