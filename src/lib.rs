//! This is a library for controlling a cursor and a screen,
//! on ANSI terminals.
//!
//!
//! ## Example
//! 
//!     use ansi_control::*;
//! 

pub fn move_cursor(i: i32, j: i32) -> String {
    ""
}

pub fn move_line(n: i32) -> String {
    ""
}

pub fn set_column(n: u32) -> String {
    ""
}

pub fn set_position(i: u32, j: u32) -> String {
    ""
}

pub fn clear_display(pos: Pos) -> String {
    ""
}

pub fn clear_line(pos: Pos) -> String {
    ""
}

pub fn scroll(n: i32) -> String {
    ""
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
