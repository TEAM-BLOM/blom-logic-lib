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
    pub fn init_xy(&mut self, x: usize, y: usize) {
        self.current = (x, y);
    }

    pub fn set_new_board(&mut self, _board: Board, x: usize, y: usize) {
        self.board = _board;
        self.current = (x, y);
    }

    pub fn check_win(&self) -> Result<bool, JsError> /* build JsError, debug i32 */ {
        let (_x, _y) = self.current;
        let mut win: bool = false;

        let dir: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (-1, 1), (1, -1)];
        let idx = self.board.convert_to_index(_x, _y);
        println!("x: {}, y: {}", _x, _y);
        println!("idx: {}", idx);
        let turn = self.board.get_board()[idx];
        if turn == 0 {
            // return Err(0);
            return Err(JsError::new("this position looks Invalid"));
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
            'direction1: loop {
                match add_usize_int32(x, dx_1) {
                    Some(_x) => x = _x,
                    None => {
                        break 'direction1
                    },
                }
                match add_usize_int32(y, dy_1) {
                    Some(_y) => y = _y,
                    None => {
                        break 'direction1
                    },
                }
                let idx = self.board.convert_to_index(x, y);

                if self.board.is_valid(x, y) && self.board.get_board()[idx] == turn {
                    cnt_1 += 1;
                } else {
                    break 'direction1;
                }
            }

            x = _x;
            y = _y;

            // direction 2
            'direction2: loop {
                match add_usize_int32(x, dx_2) {
                    Some(_x) => x = _x,
                    None => {
                        break 'direction2
                    },
                }
                match add_usize_int32(y, dy_2) {
                    Some(_y) => y = _y,
                    None => {
                        break 'direction2
                    },
                }
                let idx = self.board.convert_to_index(x, y);

                if self.board.is_valid(x, y) && self.board.get_board()[idx] == turn {
                    cnt_2 += 1;
                } else {
                    break 'direction2;
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
    
    pub fn check_33(&self) -> Result<bool, JsError> /* build JsError, debug i32 */{
        let (_x, _y) = self.current;
        let dir: [(i32, i32); 4] = [(1, 0), (0, 1), (1, 1), (-1, 1)];

        let tmp = self.board.convert_to_index(_x, _y);
        if self.board.get_board()[tmp] == -1 {
            
            return Ok(false);
        } else if self.board.get_board()[tmp] == 0 {
            // return Err(0);
            return Err(JsError::new("this position looks Invalid"));
        }

        let mut is33: bool = false;
        let mut cal33: i32 = 0;

        for i in 0..4 {
            let mut dx_1 = dir[i].0;
            let mut dy_1 = dir[i].1;
            let mut cnt_1: i32 = 0;
            let mut blk_1: bool = false;
            
            let mut x = _x;
            let mut y = _y;
            // direction 1
            'direction1: loop {
                match add_usize_int32(x, dx_1) {
                    Some(_x) => x = _x,
                    None => {
                        break 'direction1
                    },
                }
                match add_usize_int32(y, dy_1) {
                    Some(_y) => y = _y,
                    None => {
                        break 'direction1
                    },
                }
                let idx = self.board.convert_to_index(x, y);

                let (x_prev, y_prev, x_next, y_next): (usize, usize, usize, usize);
                match add_usize_int32(x, -dx_1) {
                    Some(_x) => x_prev = _x,
                    None => x_prev = 1000,
                }
                match add_usize_int32(y, -dy_1) {
                    Some(_y) => y_prev = _y,
                    None => y_prev = 1000,
                }
                match add_usize_int32(x, dx_1) {
                    Some(_x) => x_next = _x,
                    None => x_next = 1000,
                }
                match add_usize_int32(y, dy_1) {
                    Some(_y) => y_next = _y,
                    None => y_next = 1000,
                }
                let cur = self.board.get_board()[idx];
                if self.board.is_valid(x, y) {
                    if cur == 1 { // 현재 방향으로 진행하며 흑돌이면 cnt 증가
                        cnt_1 += 1;
                    } else if cur == -1 { 
                        /*
                        백돌일 경우
                            이전 돌이 빈 공간이면 막히지 않은 경우이므로 cnt 초기화 없이 loop break
                            이전 돌이 흑돌이라면 막힌 경우이므로 cnt 초기화와 loob break
                        */
                        if self.board.is_valid(x_prev, y_prev) {
                            let prev = self.board.get_board()[self.board.convert_to_index(x_prev, y_prev)];
                            if prev == 0 {
                                break 'direction1;
                            } else {
                                cnt_1 = 0;
                                break 'direction1;
                            }
                        } else {
                            cnt_1 = 0;
                            break 'direction1;
                        }
                    } else {
                        /*
                        빈 공간인 경우
                            다음 돌이 빈 공간이면 현재 돌이 더 이상 이어지지 않는게 확실하므로 loop break
                            다음 돌이 흑돌이라면 추가로 이어질 가능성이 존재하기 때문에 계속 진행

                            단, 빈 공간을 두 번째로 만난 경우에는 더 볼 것 없이 loop break
                        */
                        if blk_1 {
                            break 'direction1;
                        } else {
                            if self.board.is_valid(x_next, x_next) {
                                blk_1 = true;
                                if self.board.get_board()[self.board.convert_to_index(x_next, y_next)] == 1 {
                                    continue;
                                } else {
                                    break 'direction1;
                                }
                            } else {
                                break 'direction1;
                            }
                        }
                    }
                } else {
                    break 'direction1;
                }
            }


            dx_1 = -dir[i].0;
            dy_1 = -dir[i].1;
            let mut cnt_2: i32 = 0;
            let mut blk_2: bool = false;
            
            x = _x;
            y = _y;
            // direction 2
            'direction2: loop {
                match add_usize_int32(x, dx_1) {
                    Some(_x) => x = _x,
                    None => {
                        break 'direction2
                    },
                }
                match add_usize_int32(y, dy_1) {
                    Some(_y) => y = _y,
                    None => {
                        break 'direction2
                    },
                }
                let idx = self.board.convert_to_index(x, y);

                let (x_prev, y_prev, x_next, y_next): (usize, usize, usize, usize);
                match add_usize_int32(x, -dx_1) {
                    Some(_x) => x_prev = _x,
                    None => x_prev = 1000,
                }
                match add_usize_int32(y, -dy_1) {
                    Some(_y) => y_prev = _y,
                    None => y_prev = 1000,
                }
                match add_usize_int32(x, dx_1) {
                    Some(_x) => x_next = _x,
                    None => x_next = 1000,
                }
                match add_usize_int32(y, dy_1) {
                    Some(_y) => y_next = _y,
                    None => y_next = 1000,
                }
                let cur = self.board.get_board()[idx];
                if self.board.is_valid(x, y) {
                    if cur == 1 { // 현재 방향으로 진행하며 흑돌이면 cnt 증가
                        cnt_2 += 1;
                    } else if cur == -1 { 
                        /*
                        백돌일 경우
                            이전 돌이 빈 공간이면 막히지 않은 경우이므로 cnt 초기화 없이 loop break
                            이전 돌이 흑돌이라면 막힌 경우이므로 cnt 초기화와 loob break
                        */
                        if self.board.is_valid(x_prev, y_prev) {
                            let prev = self.board.get_board()[self.board.convert_to_index(x_prev, y_prev)];
                            if prev == 0 {
                                break 'direction2;
                            } else {
                                cnt_2 = 0;
                                break 'direction2;
                            }
                        } else {
                            cnt_2 = 0;
                            break 'direction2;
                        }
                    } else {
                        /*
                        빈 공간인 경우
                            다음 돌이 빈 공간이면 현재 돌이 더 이상 이어지지 않는게 확실하므로 loop break
                            다음 돌이 흑돌이라면 추가로 이어질 가능성이 존재하기 때문에 계속 진행

                            단, 빈 공간을 두 번째로 만난 경우에는 더 볼 것 없이 loop break
                        */
                        if blk_2 {
                            break 'direction2;
                        } else {
                            if self.board.is_valid(x_next, y_next) {
                                blk_2 = true;
                                if self.board.get_board()[self.board.convert_to_index(x_next, y_next)] == 1 {
                                    continue;
                                } else {
                                    break 'direction2;
                                }
                            } else {
                                break 'direction2;
                            }
                        }
                    }
                } else {
                    break 'direction2;
                }
            }

            if cnt_1 + cnt_2 == 2 {
                cal33 += 1;
            } else if cnt_1 == 2 && cnt_2 == 2 {
                is33 = true;
            }
            println!("{}: cnt1-{}, cnt2-{}", i, cnt_1, cnt_2);
        }

        if is33 {
            return Ok(true);
        }
        else if cal33 >= 2 {
            return Ok(true);
        }
        return Ok(false);
    }
    
}