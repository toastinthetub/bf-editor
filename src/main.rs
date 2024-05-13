mod ansii;
mod error;
mod utils;
mod render;

use std::env;
use std::fs::*;
use std::io::stdout;
use render::event_loop;
use tokio::task;
use utils::handle_arguments;
use utils::init_scr;


use crate::{ansii::{return_text_ansii, BackColor, TextColor}, error::kill, render::{TuiAsset, render}};

use crossterm::{
    cursor::MoveTo, 
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize}, 
    terminal::{self, Clear, ClearType},
    QueueableCommand, execute};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let args_clone = args.clone();
    let path = handle_arguments(args);
    let name = args_clone.get(1).expect("You must specify a filename.");
    let stdout = init_scr();

    let assets: TuiAsset = TuiAsset::new(name.to_string(), path);

    let task_handle = task::spawn(event_loop(stdout, assets));

    task_handle.await.expect("Task panicked.")
}
