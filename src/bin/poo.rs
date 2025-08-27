use crossterm::{
    execute,
    cursor::MoveTo,
    style::Print,
};
use std::{io::{stdout, Write}, thread, time::Duration};

fn main() {
    let mut out = stdout();

    // Static parts
    execute!(out,
        MoveTo(0, 0), Print("Header"),
        MoveTo(0, 2), Print("Counter: "),
    ).unwrap();

    // Update only the counter at column 9, row 2
    for i in 0..5 {
        execute!(out, MoveTo(9, 2), Print(format!("{:3}", i))).unwrap();
        out.flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
