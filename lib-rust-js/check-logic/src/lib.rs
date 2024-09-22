mod module {
    pub mod board;
    pub mod check;
}

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::module::board::Board;
    use crate::module::check::Check;

    #[test]
    fn it_works() {
        let mut array: [i32; 225] = [0; 225];
        array[0] = 1;
        array[16] = 1;
        array[32] = 1;
        // array[48] = 1;
        array[64] = 1;
        // array[80] = 1;
        // array[3] = 1;
        // array[33] = -1;
        array[94] = 1;
        array[109] = 1;
        array[139] = 1;
        let mut board = Board::new(225);
        Board::set_board(&mut board, &array);
        let check = Check::new(board, 4, 4);

        // assert_eq!(check.check_win(), Ok(true));
        assert_eq!(check.check_44(), Ok(false));
        // println!("{:?}", check.check_win())
    }
}

use wasm_bindgen::prelude::*;
use crate::module::check::Check;
use crate::module::board::Board;

#[wasm_bindgen]
pub fn check_valid_board(_board: &[i32]) -> bool {
    module::board::check_valid_board(_board)
}

#[wasm_bindgen]
pub fn init_board(_board: &[i32]) -> Result<Board, JsError> {
    if _board.len() != 225 {
        return Err(JsError::new("length is not valid"));
    }
    
    let mut board = Board::new(225);
    match board.set_board(&_board) {
        Ok(_) => return Ok(board),
        Err(e) => return Err(JsError::new(&e))
    }
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
pub fn init_obj(_board: &[i32], x: usize, y: usize) -> Result<Check, JsError> {
    if _board.len() != 225 {
        return Err(JsError::new("length is not valid"));
    }
    if x >= 15 || y >= 15 {
        return Err(JsError::new("x or y is not valid"));
    }
    
    let mut board = Board::new(225);
    match board.set_board(&_board) {
        Ok(_) => {},
        Err(e) => return Err(JsError::new(&e))
    }

    let check = Check::new(board, x, y);

    Ok(check)
}

#[wasm_bindgen]
pub fn update_check_board(mut origin: Check, new_board: &[i32], x: usize, y: usize) -> Result<Check, JsError> {
    if x >= 15 || y >= 15 {
        return Err(JsError::new("x or y is not valid"));
    }

    match origin.set_new_board(new_board, x, y) {
        Ok(_) => Ok(origin),
        Err(e) => Err(JsError::new(&e)),
    }
}

#[wasm_bindgen]
pub fn check_win(check: &Check) -> Result<bool, JsError> {
    check.check_win()
}

#[wasm_bindgen]
pub fn check_33(check: &Check) -> Result<bool, JsError> {
    check.check_33()
}

#[wasm_bindgen]
pub fn check_44(check: &Check) -> Result<bool, JsError> {
    check.check_44()
}

#[wasm_bindgen]
pub fn print_address(check: &Check) -> Result<usize, JsError> {
    let address = check as *const Check as usize;
    return Ok(address);
}