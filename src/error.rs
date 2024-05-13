use crossterm::{cursor::MoveTo, terminal::{self, Clear, ClearType}, QueueableCommand};
use crate::ansii::{return_text_ansii, TextColor, BackColor};
use std::io::{stdout, Write};

pub fn kill() {
    let mut stdout = stdout();
    let message = return_text_ansii("you've been brainfucked.\n", TextColor::Red, BackColor::White);

    let _ = stdout.queue(Clear(ClearType::All));
    let _ = stdout.flush();
    let _ = terminal::disable_raw_mode();
    let _ = stdout.queue(MoveTo(0, 0));
    let _ = stdout.flush();

    let _ = stdout.write(&message);
    std::process::exit(0);
}