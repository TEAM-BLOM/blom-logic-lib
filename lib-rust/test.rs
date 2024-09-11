fn main() {
    let mut board: Vec<i32> = Vec::new();

    for i in 0..225 {
        board.push(i);
    }

    // println!("{:?}", board);

    let mut i = 0;
    loop {
        let x = board[i] / 15;
        let y = board[i] % 15;

        println!("x: {}, y: {}", x, y);
        if x == 14 && y == 14 {
            break;
        }

        i += 1;
    }
}