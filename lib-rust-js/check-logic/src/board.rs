use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Board {
    board: Vec<i32>,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new(size: usize) -> Board {
        if size != 225 {
            panic!("Board size must be 225!");
        }

        Board {
            board: vec![0; size as usize],
        }
    }
}

pub fn check_valid_board(board: &[i32]) -> bool {
    if board.len() != 225 {
        return false;
    } 

    if board.iter().filter(|&&value| value != -1 && value != 0 && value != 1).count() > 0 {
        return false;
    }

    return true;
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Board {
            board: self.board.clone(),
        }
    }
}

impl Board {
    pub fn get_board(&self) -> &Vec<i32> {
        &self.board
    }

    pub fn set_board(&mut self, board: &[i32]) -> Result<(), String> {
        if check_valid_board(&board) {
            self.board = board.to_vec();
            return Ok(());
        } else {
            return Err("Board is invalid".to_string());
        }
    }

    pub fn convert_to_xy(&self, index: usize) -> (usize, usize) {
        let y = index / 15;
        let x = index % 15;

        (x, y)
    }

    pub fn convert_to_index(&self, x: usize, y: usize) -> usize {
        (y * 15) + x
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < 15 && y < 15
    }

    pub fn get_value(&self, x: usize, y: usize) -> i32 {
        self.board[self.convert_to_index(x, y)]
    }
}