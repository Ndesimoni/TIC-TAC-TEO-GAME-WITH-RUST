#[allow(unused_imports)]
use crate::models::Player;
use crate::models::Tabs;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

use std::io::{self, Write};

pub fn game_play(human_player: &Player) -> io::Result<(Option<Player>, bool)> {
    let mut stdout = io::stdout();

    let mut board = [' '; 9];

    let mut tabs = Tabs::<usize>::new(vec![
        (15, 9, 0),
        (19, 9, 1),
        (23, 9, 2),
        (15, 11, 3),
        (19, 11, 4),
        (23, 11, 5),
        (15, 13, 6),
        (19, 13, 7),
        (23, 13, 8),
    ]);

    loop {
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::Purge),
            SetForegroundColor(Color::Cyan),
        )?;

        print_screen(board);

        execute!(
            stdout,
            cursor::MoveTo(tabs.position().0, tabs.position().1),
            SetBackgroundColor(Color::Red),
            Print([*tabs.value()]),
            ResetColor,
            cursor::MoveTo(tabs.position().0, tabs.position().1),
        )?;

        stdout.flush()?;

        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Tab => tabs.next(),
                KeyCode::BackTab => tabs.prev(),
                KeyCode::Enter => return Ok((None, true)),
                KeyCode::Esc => return Ok((None, false)),
                _ => continue,
            }
        }
    }
}

fn print_screen(board: [char; 9]) {
    println!(
        "
     \r    +-------- TIC TAC TOE ---------+
     \r    |                              |
     \r    |    USE TAB TO MOVE CURSOR    |
     \r    |                              |
     \r    |       ENTER TO SELECT        |
     \r    |                              |
     \r    |                              |
     \r    |        +---+---+---+         |
     \r    |        | {} | {} | {} |         |
     \r    |        +---+---+---+         |
     \r    |        | {} | {} | {} |         |
     \r    |        +---+---+---+         |
     \r    |        | {} | {} | {} |         |
     \r    |        +---+---+---+         |
     \r    |                              |
     \r    |                              |
     \r    |  PRESS <ESC> TO QUIT GAME    |
     \r    |                              |
     \r    +------------------------------+
     \n\r",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    )
}
