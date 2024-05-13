use std::{io::{stdout, Stdout, Write}, path::PathBuf};

use crossterm::{
    cursor::MoveTo, 
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize}, 
    terminal::{self, Clear, ClearType},
    QueueableCommand};

pub fn handle_arguments(args: Vec<String>) -> PathBuf {
    let name = args.get(1).expect("You must specify an output file.");
    let current_dir = std::env::current_dir();
    let path = PathBuf::from(current_dir.unwrap()).join(name);
    path
}

pub fn init_scr() -> Stdout {
    let mut stdout = stdout();
    let _ = stdout.queue(Clear(ClearType::All));
    let _ = stdout.flush();
    let _ = terminal::enable_raw_mode();
    let _ = stdout.queue(MoveTo(0, 0));
    let _ = stdout.flush();

    stdout
}