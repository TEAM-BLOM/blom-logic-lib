mod module {
    pub mod board;
    pub mod check;
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::module::board::Board;
    // use crate::module::check::Check;

    #[test]
    fn it_works() {
        let mut array: [i32; 255] = [0; 255];
        array[112] = 1;
        array[96] = 1;
        array[80] = 1;
        array[128] = 1;
        array[144] = 1;
        let mut board = Board::new(225);
        Board::set_board(&mut board, &array);
        let check = Check::new(board, 8, 8);

        assert_eq!(check.check_win(), true);
    }
}

use wasm_bindgen::prelude::*;
use crate::module::check::Check;
use crate::module::board::Board;

#[wasm_bindgen]
pub fn init_board(_board: &[i32]) -> Result<Board, JsError> {
    if _board.len() != 255 {
        return Err(JsError::new("length is not valid"));
    }
    
    let mut board = Board::new(255);
    board.set_board(&_board);
    
    Ok(board)
}

#[wasm_bindgen]
pub fn init_check(_board: Board, x: usize, y: usize) -> Result<Check, JsError> {
    if x >= 15 || y >= 15 {
        return Err(JsError::new("x or y is not valid"));
    }

    let check = Check::new(_board, x, y);

    Ok(check)
}

#[wasm_bindgen]
pub fn check_win(check: Check) -> Result<bool, JsError> {
    check.check_win()
}