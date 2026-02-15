use crossterm::terminal;
use std::io;

use tic_tac_teo::screen::choose_player;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        choose_player()?;

        std::thread::sleep(std::time::Duration::from_secs(15));
        break;
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
