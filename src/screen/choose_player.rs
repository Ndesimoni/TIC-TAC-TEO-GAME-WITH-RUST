#[allow(unused_imports)]
use crate::models::Player;

use crossterm::{
    cursor, execute,
    style::{Color, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

use std::io::{self, Write};

pub fn choose_player() -> io::Result<(Player, bool)> {
    let mut stdout = io::stdout();

    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        Clear(ClearType::Purge),
        SetForegroundColor(Color::Cyan)
    )?;

    print_screen();

    stdout.flush()?;

    Ok((Player::X, true))
}

fn print_screen() {
    println!(
        "
     \r    +-------- TIC TAC TOE ---------+
     \r    |                              |
     \r    |    USE TAB TO MOVE CURSOR    |
     \r    |                              |
     \r    |       ENTER TO SELECT        |
     \r    |                              |
     \r    |                              |
     \r    |  +-----------------------+   |
     \r    |  |  CHOOSE YOUR PLAYER   |   |
     \r    |  +-----------------------+   |
     \r    |                              |
     \r    |                              |
     \r    |           <X>  <O>           |
     \r    |                              |
     \r    |                              |
     \r    |  PRESS <ESC> TO QUIT GAME    |
     \r    |                              |
     \r    +------------------------------+
     \n\r"
    );
}
