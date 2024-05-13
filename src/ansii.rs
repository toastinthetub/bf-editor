// \x1b[{FRONT};{BACK}m        set fg bg
// \x1b[0m                     clear

const BLACK: u8 = 30;
const RED: u8 = 31;
const GREEN: u8 = 32;
const YELLOW: u8 = 33;
const BLUE: u8 = 34;
const MAGENTA: u8 = 35;
const CYAN: u8 = 36;
const WHITE: u8 = 37;

const BG_BLACK: u8 = 40;
const BG_RED: u8 = 41;
const BG_GREEN: u8 = 42;
const BG_YELLOW: u8 = 43;
const BG_BLUE: u8 = 44;
const BG_MAGENTA: u8 = 45;
const BG_CYAN: u8 = 46;
const BG_WHITE: u8 = 47;

const NONE: u8 = 0;
pub enum TextColor {
    White,
    Black,
    Blue, 
    Red,
    Cyan,
    Magenta,
    None,
}

pub enum BackColor {
    White,
    Black,
    Blue, 
    Red,
    Cyan,
    Magenta,
    None
} 

pub fn return_text_ansii(text: &str, front: TextColor, back: BackColor) -> Vec<u8> {
    let front_color_code = match front {
        TextColor::White => WHITE,
        TextColor::Black => BLACK,
        TextColor::Blue => BLUE,
        TextColor::Red => RED,
        TextColor::Cyan => CYAN,
        TextColor::Magenta => MAGENTA,
        TextColor::None => NONE
        
    };

    let back_color_code = match back {
        BackColor::White => BG_WHITE,
        BackColor::Black => BG_BLACK,
        BackColor::Blue => BG_BLUE,
        BackColor::Red => BG_RED,
        BackColor::Cyan => BG_CYAN,
        BackColor::Magenta => BG_MAGENTA,
        BackColor::None => NONE
    };

    let ansii_sequence = format!("\x1b[{};{}m{}{}", front_color_code, back_color_code, text, "\x1b[0m");
    ansii_sequence.into_bytes()
}