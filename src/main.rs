use std::io;

fn main() {
    let mut board = [' '; 9];
    print_board(board);

    loop {
        print!("Enter position for X: ");
        let index = get_index_from_input();

        if let Err(e) = index {
            println!("{e}");

            continue;
        }
        let index = index.unwrap();

        if let None = index {
            break;
        }

        let index = index.unwrap();
        board[index] = 'X';

        print_board(board);
    }
}

fn print_board(board: [char; 9]) {
    println!(
        "
    +------+------+------+
    |  {}   |  {}   |  {}   |
    |      |      |      |
    +------+------+------+
    |  {}   |  {}   |  {}   |
    |      |      |      |
    +------+------+------+
    |  {}   |  {}   |  {}   |
    |      |      |      |
    +------+------+------+
    ",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    );
}

fn get_index_from_input() -> Result<Option<usize>, String> {
    let mut input = String::new();

    let _ = io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    let input = input.trim();

    if input == "quite" {
        return Ok(None);
    }

    let index = input
        .parse::<usize>()
        .map_err(|_| format!("Input should,be  and integer"))?;

    if index < 1 || index > 9 {
        return Err(format!("the position should be an integer from 1 to 9"));
    }

    Ok(Some(index - 1))
}
