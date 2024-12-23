/* tslint:disable */
/* eslint-disable */
export function setup_detector(min_face_sizes: number, score_thresh: number, pyramid_scale_factor: number, slide_window_step: number): void;
export function detect_bounding_box(rgba: Uint8Array, width: number, height: number, block_size: number, is_mosaic: boolean, overlay_image: Uint8Array): (BboxInfo)[];
export class BboxInfo {
  private constructor();
  free(): void;
  x(): number;
  y(): number;
  static new(x: number, y: number, mosaic: (Row)[]): BboxInfo;
  readonly mosaic: (Row)[];
}
export class Rgb {
  private constructor();
  free(): void;
  readonly r: number;
  readonly g: number;
  readonly b: number;
}
export class Row {
  private constructor();
  free(): void;
  cols(): (Rgb)[];
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_bboxinfo_free: (a: number, b: number) => void;
  readonly bboxinfo_x: (a: number) => number;
  readonly bboxinfo_y: (a: number) => number;
  readonly bboxinfo_mosaic: (a: number, b: number) => void;
  readonly bboxinfo_new: (a: number, b: number, c: number, d: number) => number;
  readonly __wbg_row_free: (a: number, b: number) => void;
  readonly row_cols: (a: number, b: number) => void;
  readonly __wbg_rgb_free: (a: number, b: number) => void;
  readonly rgb_r: (a: number) => number;
  readonly rgb_g: (a: number) => number;
  readonly rgb_b: (a: number) => number;
  readonly setup_detector: (a: number, b: number, c: number, d: number) => void;
  readonly detect_bounding_box: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export_0: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_1: (a: number, b: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
