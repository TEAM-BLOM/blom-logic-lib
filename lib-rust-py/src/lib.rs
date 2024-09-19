use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

mod board;
mod check;

use board::Board;
use check::Check;

#[pyfunction]
pub fn check_valid_board_out( _board: Vec<i32>) -> PyResult<bool> {
    Ok(board::check_valid_board(&_board))
}

#[pyfunction]
pub fn init_board(_board: Vec<i32>) -> Result<Board, PyErr> {
    if _board.len() != 225 {
        return Err(PyErr::new::<PyValueError, _>("length is not valid"));
    }
    
    let mut board = Board::new(225);
    match board.set_board(&_board) {
        Ok(_) => return Ok(board),
        Err(state) => return Err(PyErr::new::<PyValueError, _>(state.to_string()))
    }
}

#[pyfunction]
pub fn init_check(_board: Board, x: usize, y: usize) -> Result<Check, PyErr> {
    if x >= 15 || y >= 15 {
        return Err(PyErr::new::<PyValueError, _>("x or y is not valid"));
    }

    let check = Check::new(&_board, x, y);

    Ok(check)
}

#[pyfunction]
pub fn update_check_board(mut origin: Check, new_board: Vec<i32>, x: usize, y: usize) -> Result<Check, PyErr> {
    if x >= 15 || y >= 15 {
        return Err(PyErr::new::<PyValueError, _>("x or y is not valid"));
    }

    match origin.set_new_board(&new_board, x, y) {
        Ok(_) => Ok(origin),
        Err(e) => Err(PyErr::new::<PyValueError, _>(e.to_string())),
    }
}

#[pyfunction]
pub fn check_win(check: &Check) -> Result<bool, PyErr> {
    check.check_win()
}

#[pyfunction]
pub fn check_33(check: &Check) -> Result<bool, PyErr> {
    check.check_33()
}

#[pyfunction]
pub fn check_44(check: &Check) -> Result<bool, PyErr> {
    check.check_44()
}

#[pyfunction]
pub fn print_address(check: &Check) -> Result<usize, PyErr> {
    let address = check as *const Check as usize;
    return Ok(address);
}

#[pymodule]
pub fn server_ai(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(check_valid_board_out, m)?)?;
    m.add_function(wrap_pyfunction!(init_board, m)?)?;
    m.add_function(wrap_pyfunction!(init_check, m)?)?;
    m.add_function(wrap_pyfunction!(update_check_board, m)?)?;
    m.add_function(wrap_pyfunction!(check_win, m)?)?;
    m.add_function(wrap_pyfunction!(check_33, m)?)?;
    m.add_function(wrap_pyfunction!(check_44, m)?)?;
    m.add_function(wrap_pyfunction!(print_address, m)?)?;
    Ok(())
}