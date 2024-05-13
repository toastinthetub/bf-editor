use crossterm::{cursor::MoveTo, event::{poll, read, Event, EventStream, KeyCode, KeyModifiers}, terminal::{self, Clear, ClearType}, QueueableCommand};
use tokio::time;
use std::{fs::File, io::{stdout, BufWriter, Stdout, Write}, path::PathBuf, process, sync::Arc, thread::sleep, time::{Duration, Instant}};
use ropey::{Rope};
use lazy_static::lazy_static;

use crate::{ansii::{return_text_ansii, BackColor, TextColor}, error::kill};

lazy_static! {
    static ref VERT_CHAR: Vec<u8> = return_text_ansii("██", TextColor::Cyan, BackColor::Cyan);
    static ref HORIZ_CHAR: Vec<u8> = return_text_ansii("█", TextColor::Cyan, BackColor::Cyan);
}

#[derive(Clone)]
pub struct TuiAsset {
    pub name: String,
    pub name_bytes: Vec<u8>,
    pub banner: Vec<u8>,
    
    pub text: Rope,
    pub path: PathBuf,

    pub selected: u32,

    pub cursorpos: (u16, u16),

    pub single: Vec<u8>,
    pub double: Vec<u8>,

    pub w: u16,
    pub h: u16
}

impl TuiAsset {
    pub fn new(name: String, path: PathBuf) -> Self {
        let name_bytes = return_text_ansii(format!("EDITING: {}", name).as_str(), TextColor::Black, BackColor::Cyan);
        let banner = return_text_ansii("BFScribble - The Brainfuck-only text editor.", TextColor::Red, BackColor::Cyan);
        let single = return_text_ansii("█", TextColor::Cyan, BackColor::Cyan);
        let double = return_text_ansii("██", TextColor::Cyan, BackColor::Cyan);
        let (w, h) = terminal::size().unwrap();
        let text = Rope::new();

        Self {
            name,
            name_bytes,
            path,
            banner,
            text,
            selected: 0,
            cursorpos: (0, 0),
            single,
            double,
            w,
            h
        }

    }
}

pub async fn event_loop(mut stdout: Stdout, mut assets: TuiAsset) {
    loop {
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.flush().unwrap();
        if poll_event().await {
            if let Ok(event) = read() {
                match event {
                    Event::Resize(nw, nh) => {
                        assets.w = nw;
                        assets.h = nh;
                        render(&mut stdout, assets.clone()).await;
                    }
                    Event::Key(event) => {
                        render(&mut stdout, assets.clone()).await;
                        match event.code {
                            KeyCode::Char(x) => match x {
                                'c' if event.modifiers.contains(KeyModifiers::CONTROL) => kill(),
                                's' if event.modifiers.contains(KeyModifiers::CONTROL) => {
                                    let _ = assets.text.write_to(
                                        BufWriter::new(File::create(assets.path.clone()).unwrap())
                                    );
                                },
                                '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => {
                                    assets.text.append(Rope::from_str(x.to_string().as_str()));
                                    assets.cursorpos.0 += 1;
                                }
                                _ => {}
                            }
                            KeyCode::Backspace => {
                                if assets.text.len_chars() > 0 {
                                    assets.text.remove(assets.text.len_chars() - 1..)
                                } else {}
                            }
                            KeyCode::Enter => {
                                assets.text.append(Rope::from_str("\n"));
                                assets.cursorpos.0 = 1;
                                assets.cursorpos.1 += 1;
                            }
                            KeyCode::Left => {
                                
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        render(&mut stdout, assets.clone()).await;
                    }
                }
            }
        } else {
            render(&mut stdout, assets.clone()).await;
        }
        time::sleep(Duration::from_millis(33)).await;
    }
}

async fn poll_event() -> bool {
    if let Ok(true) = poll(Duration::from_secs(0)) {
        return true; 
    }
    false
}


pub async fn render(stdout: &mut Stdout, mut assets: TuiAsset) {
    let _ = stdout.queue(Clear(ClearType::All));
    draw_box(stdout, &assets);
    let _ = stdout.queue(MoveTo(2, 1));
    let _ = stdout.flush();

    for (index, line) in assets.text.lines().enumerate() {
        let _ = stdout.queue(MoveTo(2, index as u16 + 1));
        print!("{}", line);
        let _ = stdout.flush().unwrap();
        assets.cursorpos.0 = line.len_chars() as u16 + 1;
        assets.cursorpos.1 = index as u16;
    }

    let _ = stdout.queue(MoveTo(assets.cursorpos.0 + 1, assets.cursorpos.1 + 1));
    let _ = stdout.flush();
}

fn draw_box(stdout: &mut Stdout, assets: &TuiAsset) {
    vertical_bar(stdout, &VERT_CHAR, 0, assets.w - assets.w, assets.h);
    vertical_bar(stdout, &VERT_CHAR, assets.w - 2, assets.w - assets.w, assets.h);
    horizontal_bar(stdout, &HORIZ_CHAR, assets.w - assets.w, assets.h - assets.h, assets.w);
    horizontal_bar(stdout, &HORIZ_CHAR, assets.w - assets.w, assets.h, assets.w);
    
    let _ = stdout.queue(MoveTo(assets.w - assets.w + 2, assets.h - assets.h));
    let _ = stdout.flush();
    let _ = stdout.write(&assets.banner);
    let _ = stdout.flush();
    let _ = stdout.queue(MoveTo(assets.w - assets.w + 2, assets.h));
    let _ = stdout.flush();
    let _ = stdout.write(&assets.name_bytes);
    let _ = stdout.flush();
}

pub fn vertical_bar(mut stdout: &Stdout, char: &Vec<u8>,  x: u16, start_y: u16, end_y: u16) {
    let bar_height = end_y - start_y;

    stdout.queue(MoveTo(x, start_y)).unwrap();
    stdout.flush().unwrap();
    for cell in 0..=bar_height {
        stdout.write(char).unwrap();
        stdout.queue(MoveTo(x, cell)).unwrap();
        stdout.flush().unwrap();
    }
}

pub fn horizontal_bar(mut stdout: &Stdout, char: &Vec<u8>, x: u16, y: u16, length: u16) { 
    stdout.queue(MoveTo(x, y)).unwrap();
    stdout.flush().unwrap();
    for cell in 0..=length {
        stdout.write(char).unwrap();
        stdout.queue(MoveTo(cell, y)).unwrap();
        stdout.flush().unwrap();
    }
}

