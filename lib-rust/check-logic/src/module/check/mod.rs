use crate::module::board::Board;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Check {
    board: Board,
    current: (usize, usize), // x, y
}

fn add_usize_int32(a: usize, b: i32) -> Option<usize> {
    if b >= 0 {
        Some(a + b as usize)
    } else {
        let b_abs = b.abs() as usize;
        if a < b_abs {
            None
        } else {
            Some(a - b_abs)
        }
    }
}

#[wasm_bindgen]
impl Check {
    #[wasm_bindgen(constructor)]
    pub fn new(_board: Board, x: usize, y: usize) -> Check {
        if x >= 15 || y >= 15 {
            panic!("Invalid input");
        }

        Check {
            board: _board,
            current: (x, y),
        }
    }
}

impl Check {
    pub fn initXY(&mut self, x: usize, y: usize) {
        self.current = (x, y);
    }

    pub fn set_new_board(&mut self, _board: Board, x: usize, y: usize) {
        self.board = _board;
        self.current = (x, y);
    }

    pub fn check_win(&self) -> Result<bool, JsError> {
        let (_x, _y) = self.current;
        let mut win: bool = false;

        let dir: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (-1, 1), (1, -1)];
        let idx = self.board.convert_to_index(_x, _y);
        println!("x: {}, y: {}", _x, _y);
        println!("idx: {}", idx);
        let turn = self.board.get_board()[idx];
        if turn == 0 {
            return Err(JsError::new("this position looks Invalid"))
        }

        for i in (0..8).step_by(2) {
            let dx_1 = dir[i].0;
            let dy_1 = dir[i].1;
            let mut cnt_1: i32 = 0;
            let dx_2 = dir[i + 1].0;
            let dy_2 = dir[i + 1].1;
            let mut cnt_2: i32 = 0;

            let mut x = _x;
            let mut y = _y;

            // direction 1
            loop {
                x = add_usize_int32(x, dx_1).unwrap();
                y = add_usize_int32(y, dy_1).unwrap();
                let idx = self.board.convert_to_index(x, y);

                if self.board.is_valid(x, y) && self.board.get_board()[idx] == turn {
                    cnt_1 += 1;
                } else {
                    break;
                }
            }

            x = _x;
            y = _y;

            // direction 2
            loop {
                x = add_usize_int32(x, dx_2).unwrap();
                y = add_usize_int32(y, dy_2).unwrap();
                let idx = self.board.convert_to_index(x, y);

                if self.board.is_valid(x, y) && self.board.get_board()[idx] == turn {
                    cnt_2 += 1;
                } else {
                    break;
                }
            }

            if turn == 1 && (cnt_1 + cnt_2 == 4) {
                win = true;
            } else if turn == -1 && ((cnt_1 + cnt_2 == 4) || (cnt_1 + cnt_2 == 5)) {
                win = true;
            }

            if win {
                break;
            }
        }

        return Ok(win);
    }

    pub fn check_33(&self) -> Result<bool, JsError> {
        let (_x, _y) = self.current;
        let dir: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (-1, 1), (1, -1)];

        let tmp = self.board.convert_to_index(_x, _y);
        if self.board.get_board()[tmp] == -1 {
            return Ok(false);
        } else if self.board.get_board()[tmp] == 0 {
            return Err(JsError::new("this position looks Invalid"));
        }

        for i in (0..8).step_by(2) {
            let dx_1 = dir[i].0;
            let dy_1 = dir[i].1;
            let mut cnt_1: i32 = 0;
            
            let mut x = _x;
            let mut y = _y;
            // direction 1
            loop {
                x = add_usize_int32(x, dx_1).unwrap();
                y = add_usize_int32(y, dy_1).unwrap();
                let idx = self.board.convert_to_index(x, y);

                if self.board.is_valid(x, y) && self.board.get_board()[idx] == 1 {
                    cnt_1 += 1;
                } else {
                    break;
                }
            }
        }


    }
}