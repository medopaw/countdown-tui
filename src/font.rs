use std::collections::HashMap;

pub type CharMap = Vec<&'static str>;

pub fn get_font() -> HashMap<char, CharMap> {
    let mut font = HashMap::new();
    
    font.insert(':', vec![
        "   ",
        "██╗",
        "╚═╝",
        "██╗",
        "╚═╝",
        "   ",
    ]);
    
    font.insert('0', vec![
        " ██████╗ ",
        "██╔═████╗",
        "██║██╔██║",
        "████╔╝██║",
        "╚██████╔╝",
        " ╚═════╝ ",
    ]);
    
    font.insert('1', vec![
        " ██╗",
        "███║",
        "╚██║",
        " ██║",
        " ██║",
        " ╚═╝",
    ]);
    
    font.insert('2', vec![
        "██████╗ ",
        "╚════██╗",
        " █████╔╝",
        "██╔═══╝ ",
        "███████╗",
        "╚══════╝",
    ]);
    
    font.insert('3', vec![
        "██████╗ ",
        "╚════██╗",
        " █████╔╝",
        " ╚═══██╗",
        "██████╔╝",
        "╚═════╝ ",
    ]);
    
    font.insert('4', vec![
        "██╗  ██╗",
        "██║  ██║",
        "███████║",
        "╚════██║",
        "     ██║",
        "     ╚═╝",
    ]);
    
    font.insert('5', vec![
        "███████╗",
        "██╔════╝",
        "███████╗",
        "╚════██║",
        "███████║",
        "╚══════╝",
    ]);
    
    font.insert('6', vec![
        " ██████╗ ",
        "██╔════╝ ",
        "███████╗ ",
        "██╔═══██╗",
        "╚██████╔╝",
        " ╚═════╝ ",
    ]);
    
    font.insert('7', vec![
        "███████╗",
        "╚════██║",
        "    ██╔╝",
        "   ██╔╝ ",
        "   ██║  ",
        "   ╚═╝  ",
    ]);
    
    font.insert('8', vec![
        " █████╗ ",
        "██╔══██╗",
        "╚█████╔╝",
        "██╔══██╗",
        "╚█████╔╝",
        " ╚════╝ ",
    ]);
    
    font.insert('9', vec![
        " █████╗ ",
        "██╔══██╗",
        "╚██████║",
        " ╚═══██║",
        " █████╔╝",
        " ╚════╝ ",
    ]);
    
    font
}

pub fn get_paused_text() -> CharMap {
    vec![
        "█▀▄ ▄▀▄ █ █ ▄▀▀ ██▀ █▀▄",
        "█▀  █▀█ ▀▄█ ▄██ █▄▄ █▄▀",
    ]
}


pub fn char_height() -> usize {
    6
}

pub fn paused_text_width() -> usize {
    let paused = get_paused_text();
    paused[0].chars().count()
}

pub fn paused_text_height() -> usize {
    get_paused_text().len()
}