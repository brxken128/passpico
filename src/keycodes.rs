// modifier bytes
pub const KEY_MOD_LCTRL: u8 = 0x01;
pub const KEY_MOD_LSHIFT: u8 = 0x02;
pub const KEY_MOD_LALT: u8 = 0x04;
pub const KEY_MOD_LMETA: u8 = 0x08;
pub const KEY_MOD_RCTRL: u8 = 0x10;
pub const KEY_MOD_RSHIFT: u8 = 0x20;
pub const KEY_MOD_RALT: u8 = 0x40;
pub const KEY_MOD_RMETA: u8 = 0x80;
pub const KEY_MOD_NONE: u8 = 0x00;

pub const LSHIFT_CHARS: &str = "!@#$%^&*()_+{}|~:\"<>?";

// scan codes
pub const KEY_NONE: u8 = 0x00;
pub const KEY_ERR_OVF: u8 = 0x01;
pub const KEY_A: u8 = 0x04;
pub const KEY_B: u8 = 0x05;
pub const KEY_C: u8 = 0x06;
pub const KEY_D: u8 = 0x07;
pub const KEY_E: u8 = 0x08;
pub const KEY_F: u8 = 0x09;
pub const KEY_G: u8 = 0x0a;
pub const KEY_H: u8 = 0x0b;
pub const KEY_I: u8 = 0x0c;
pub const KEY_J: u8 = 0x0d;
pub const KEY_K: u8 = 0x0e;
pub const KEY_L: u8 = 0x0f;
pub const KEY_M: u8 = 0x10;
pub const KEY_N: u8 = 0x11;
pub const KEY_O: u8 = 0x12;
pub const KEY_P: u8 = 0x13;
pub const KEY_Q: u8 = 0x14;
pub const KEY_R: u8 = 0x15;
pub const KEY_S: u8 = 0x16;
pub const KEY_T: u8 = 0x17;
pub const KEY_U: u8 = 0x18;
pub const KEY_V: u8 = 0x19;
pub const KEY_W: u8 = 0x1a;
pub const KEY_X: u8 = 0x1b;
pub const KEY_Y: u8 = 0x1c;
pub const KEY_Z: u8 = 0x1d;

pub const KEY_1: u8 = 0x1e;
pub const KEY_2: u8 = 0x1f;
pub const KEY_3: u8 = 0x20;
pub const KEY_4: u8 = 0x21;
pub const KEY_5: u8 = 0x22;
pub const KEY_6: u8 = 0x23;
pub const KEY_7: u8 = 0x24;
pub const KEY_8: u8 = 0x25;
pub const KEY_9: u8 = 0x26;
pub const KEY_0: u8 = 0x27;

pub const KEY_ENTER: u8 = 0x28;
pub const KEY_ESC: u8 = 0x29;
pub const KEY_BACKSPACE: u8 = 0x2a;
pub const KEY_TAB: u8 = 0x2b;
pub const KEY_SPACE: u8 = 0x2c;
pub const KEY_MINUS: u8 = 0x2d;
pub const KEY_EQUAL: u8 = 0x2e;
pub const KEY_LEFTBRACE: u8 = 0x2f;
pub const KEY_RIGHTBRACE: u8 = 0x30;
pub const KEY_BACKSLASH: u8 = 0x31;
pub const KEY_HASHTILDE: u8 = 0x32;
pub const KEY_SEMICOLON: u8 = 0x33;
pub const KEY_APOSTROPHE: u8 = 0x34;
pub const KEY_GRAVE: u8 = 0x35;
pub const KEY_COMMA: u8 = 0x36;
pub const KEY_DOT: u8 = 0x37;
pub const KEY_SLASH: u8 = 0x38;
pub const KEY_CAPSLOCK: u8 = 0x39;

// this converts a char to a keycode
#[must_use]
pub const fn char_to_keycode(c: char) -> u8 {
    match c {
        'a' | 'A' => KEY_A,
        'b' | 'B' => KEY_B,
        'c' | 'C' => KEY_C,
        'd' | 'D' => KEY_D,
        'e' | 'E' => KEY_E,
        'f' | 'F' => KEY_F,
        'g' | 'G' => KEY_G,
        'h' | 'H' => KEY_H,
        'i' | 'I' => KEY_I,
        'j' | 'J' => KEY_J,
        'k' | 'K' => KEY_K,
        'l' | 'L' => KEY_L,
        'm' | 'M' => KEY_M,
        'n' | 'N' => KEY_N,
        'o' | 'O' => KEY_O,
        'p' | 'P' => KEY_P,
        'q' | 'Q' => KEY_Q,
        'r' | 'R' => KEY_R,
        's' | 'S' => KEY_S,
        't' | 'T' => KEY_T,
        'u' | 'U' => KEY_U,
        'v' | 'V' => KEY_V,
        'w' | 'W' => KEY_W,
        'x' | 'X' => KEY_X,
        'y' | 'Y' => KEY_Y,
        'z' | 'Z' => KEY_Z,
        '1' | '!' => KEY_1,
        '2' | '@' => KEY_2,
        '3' | '#' => KEY_3,
        '4' | '$' => KEY_4,
        '5' | '%' => KEY_5,
        '6' | '^' => KEY_6,
        '7' | '&' => KEY_7,
        '8' | '*' => KEY_8,
        '9' | '(' => KEY_9,
        '0' | ')' => KEY_0,
        '-' | '_' => KEY_MINUS,
        '=' | '+' => KEY_EQUAL,
        '[' | '{' => KEY_LEFTBRACE,
        ']' | '}' => KEY_RIGHTBRACE,
        '\\' | '|' => KEY_BACKSLASH,
        ';' | ':' => KEY_SEMICOLON,
        '\'' | '"' => KEY_APOSTROPHE,
        '`' | '~' => KEY_GRAVE,
        ',' | '<' => KEY_COMMA,
        '.' | '>' => KEY_DOT,
        '/' | '?' => KEY_SLASH,
        ' ' => KEY_SPACE,
        _ => unreachable!(),
    }
}

#[must_use]
pub fn get_modifier(c: char) -> u8 {
    if c.is_uppercase() {
        return KEY_MOD_LSHIFT;
    }

    if LSHIFT_CHARS.chars().any(|z| c == z) {
        return KEY_MOD_LSHIFT;
    }

    KEY_MOD_NONE
}
