/* tslint:disable */
/* eslint-disable */
/**
* @param {Int32Array} _board
* @returns {boolean}
*/
export function check_valid_board(_board: Int32Array): boolean;
/**
* @param {Int32Array} _board
* @returns {Board}
*/
export function init_board(_board: Int32Array): Board;
/**
* @param {Board} _board
* @param {number} x
* @param {number} y
* @returns {Check}
*/
export function init_check(_board: Board, x: number, y: number): Check;
/**
* @param {Check} origin
* @param {Int32Array} new_board
* @param {number} x
* @param {number} y
* @returns {Check}
*/
export function update_check_board(origin: Check, new_board: Int32Array, x: number, y: number): Check;
/**
* @param {Check} check
* @returns {boolean}
*/
export function check_win(check: Check): boolean;
/**
* @param {Check} check
* @returns {boolean}
*/
export function check_33(check: Check): boolean;
/**
* @param {Check} check
* @returns {boolean}
*/
export function check_44(check: Check): boolean;
/**
* @param {Check} check
* @returns {number}
*/
export function print_address(check: Check): number;
/**
*/
export class Board {
  free(): void;
/**
* @param {number} size
*/
  constructor(size: number);
}
/**
*/
export class Check {
  free(): void;
/**
* @param {Board} _board
* @param {number} x
* @param {number} y
*/
  constructor(_board: Board, x: number, y: number);
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_board_free: (a: number, b: number) => void;
  readonly board_new: (a: number) => number;
  readonly __wbg_check_free: (a: number, b: number) => void;
  readonly check_new: (a: number, b: number, c: number) => number;
  readonly check_valid_board: (a: number, b: number) => number;
  readonly init_board: (a: number, b: number, c: number) => void;
  readonly init_check: (a: number, b: number, c: number, d: number) => void;
  readonly update_check_board: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly check_win: (a: number, b: number) => void;
  readonly check_33: (a: number, b: number) => void;
  readonly check_44: (a: number, b: number) => void;
  readonly print_address: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
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
