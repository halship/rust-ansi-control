//! This is a library for controlling a cursor and a screen,
//! on ANSI terminals.
//!
//!
//! ## Example
//! 
//!     use ansi_control::*;
//!     
//!     println!("test 0");
//!     print!("{}", set_column(1));
//!     println!("test 1");
//!     print!("{}", clear_display(Pos::Both));
//! 

use std::cmp::{self, Ordering};

/// A Pos is position of clearing (display|line) from cursor.
pub enum Pos {
    Back,
    Front,
    Both,
}

impl Pos {
    fn num(&self) -> u32 {
        match *self {
            Pos::Back   => 0,
            Pos::Front  => 1,
            Pos::Both   => 2,
        }
    }
}

/// Moves the cursor _i_ (row), _j_ (column) cells. If the cursor
/// is already at the edge of the screen, this has no effect.
pub fn move_cursor(i: i32, j: i32) -> String {
    let code0 = match i.cmp(&0) {
        Ordering::Less      => format!("\x1B[{}B", -i),
        Ordering::Equal     => format!(""),
        Ordering::Greater   => format!("\x1B[{}A", i),
    };
    let code1 = match j.cmp(&0) {
        Ordering::Less      => format!("\x1B[{}D", -j),
        Ordering::Equal     => format!(""),
        Ordering::Greater   => format!("\x1B[{}C", j),
    };
    format!("{}{}", code0, code1)
}

/// Moves the cursor to beginning of the line _n_ lines down.
/// If _n_ is a negative number, this function moves the cursor
/// _|n|_ lines up.
pub fn move_line(n: i32) -> String {
    match n.cmp(&0) {
        Ordering::Less      => format!("\x1B[{}F", -n),
        Ordering::Equal     => format!(""),
        Ordering::Greater   => format!("\x1B[{}E", n),
    }
}

/// Sets the column of the cursor. (n: column)
pub fn set_column(n: u32) -> String {
    format!("\x1B[{}G", cmp::max(1, n))
}

/// Sets the position of the cursor. (i: row, j: column)
pub fn set_position(i: u32, j: u32) -> String {
    format!("\x1B[{};{}H", cmp::max(1, i), cmp::max(1, j))
}

/// Clears part of screen. If pos is Pos::Back, clear from cursor
/// to the end of the screen. If pos is Pos::Front, clear from
/// cursor to beginning of the screen. If pos is Pos::Both, clear
/// entire screen.
pub fn clear_display(pos: Pos) -> String {
    format!("\x1B[{}J", pos.num())
}

/// Clears part of line. If pos is Pos::Back, clear from cursor
/// to the end of the line. If pos is Pos::Front, clear from
/// cursor to beginning of the line. If pos is Pos::Both, clear
/// entire line.
pub fn clear_line(pos: Pos) -> String {
    format!("\x1B[{}K", pos.num())
}

/// Scroll whole page up by _n_ lines. If _n_ is a negative
/// number, this function scroll whole page down by _|n|_ lines.
pub fn scroll(n: i32) -> String {
    match n.cmp(&0) {
        Ordering::Less      => format!("\x1B[{}T", -n),
        Ordering::Equal     => format!(""),
        Ordering::Greater   => format!("\x1B[{}S", n),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    macro_rules! test {
        ($name: ident; $ctrl: expr => $result: expr) => {
            #[test]
            fn $name() {
                assert_eq!($result, $ctrl)
            }
        };
    }
    
    test!(move_cursor0; move_cursor(2, -4) => "\x1B[2A\x1B[4D");
    test!(move_cursor1; move_cursor(-2, 0) => "\x1B[2B");
    test!(move_cursor2; move_cursor(0, 6) => "\x1B[6C");
    test!(move_cursor3; move_cursor(0, 0) => "");
    
    test!(move_line0; move_line(4) => "\x1B[4E");
    test!(move_line1; move_line(-4) => "\x1B[4F");
    
    test!(set_column0; set_column(3) => "\x1B[3G");
    test!(set_column1; set_column(0) => "\x1B[1G");
    
    test!(set_position0; set_position(3, 5) => "\x1B[3;5H");
    test!(set_position1; set_position(0, 0) => "\x1B[1;1H");
    
    test!(clear_display0; clear_display(Pos::Back) => "\x1B[0J");
    test!(clear_display1; clear_display(Pos::Front) => "\x1B[1J");
    test!(clear_display2; clear_display(Pos::Both) => "\x1B[2J");
    
    test!(clear_line0; clear_line(Pos::Back) => "\x1B[0K");
    test!(clear_line1; clear_line(Pos::Front) => "\x1B[1K");
    test!(clear_line2; clear_line(Pos::Both) => "\x1B[2K");
    
    test!(scroll0; scroll(7) => "\x1B[7S");
    test!(scroll1; scroll(-5) => "\x1B[5T");
}
