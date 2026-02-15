#[allow(unused_imports)]
use crate::models::Player;

use std::io;

pub fn choose_player() -> io::Result<(Player, bool)> {
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

    Ok((Player::X, true))
}
