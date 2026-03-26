use std::io;
use std::io::Write;
use termion::raw::IntoRawMode;//Трейт для перевода терминала в raw-режим

mod display;mod currect;mod input;
use display::*;
use currect::*;
use input::*;

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut s:i32 = 1;
    let mut p:String = String::new();
    let mut m:i32 = 0;
    let mut program_state:u8 = 1;

    loop {
        write!(stdout, "\x1b[2J\x1b[3J\x1b[1;1H").unwrap();

        display(&mut stdout,p.clone());
        draw_buttons(s,&mut stdout);

        stdout.flush().unwrap();

        match program_state{
            0 => if !input_mode(){break;},
            1 => if !input_calc(&mut s, &mut p){break;},
            _ => (),
        }
    }
}