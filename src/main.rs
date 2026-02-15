use crossterm::{
    cursor, execute,
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};

use tic_tac_teo::screen::choose_player;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    loop {
        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All),
            Clear(ClearType::Purge)
        )?;

        choose_player()?;

        std::thread::sleep(std::time::Duration::from_secs(10));
        break;
    }

    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        Clear(ClearType::All),
        Clear(ClearType::Purge)
    )?;

    stdout.flush()?;

    terminal::disable_raw_mode()?;
    Ok(())
}
