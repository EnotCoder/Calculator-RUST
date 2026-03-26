use std::io;
use std::io::Write;

use crate::currect::*;
use crate::input::*;

const ENTER:&str = "\x1b[G";
const RED:&str = "\x1b[31m";
const RESET:&str = "\x1b[0m";

pub fn draw_buttons<W: Write>(s: i32, mut stdout: W){
    write!(stdout," ===============\n{ENTER}");
    for i in 0..4{
        write!(stdout," | ");
        for j in 0..4{
            let num = j+1+i*4;
            let ch = if num == s{
                let n = get_currect_symbol(num);
                RED.to_owned() + &n
                + &RESET.to_owned()
            }else{
                get_currect_symbol(num)
            };
            write!(stdout,"{0} ",ch);
        };
        write!(stdout,"|{0}|\n{ENTER}",get_currect_mode(i));
    }

    write!(stdout," ===============\n{ENTER}");
}

pub fn display<W: Write>(mut stdout: W,text:String){
    write!(stdout," ===============\n{ENTER}");
    write!(stdout," |");
    write!(stdout,"{text}");
    for i in 0..9-text.len() as usize{
        write!(stdout," ");
    }
    write!(stdout,"|   |");
    write!(stdout,"\n{ENTER}");
}