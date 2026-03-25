use std::io;
use std::io::Write;
use termion::event::Key;//считывание с клавиратуры
use termion::input::TermRead;//Трейт для чтения событий клавиатуры
use termion::raw::IntoRawMode;//Трейт для перевода терминала в raw-режим
use evalexpr::*;

const ENTER:&str = "\x1b[G";

const RED:&str = "\x1b[31m";
const RESET:&str = "\x1b[0m";

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut s:i32 = 1;
    let mut p:String = String::new();

    loop {
        write!(stdout, "\x1b[2J\x1b[3J\x1b[1;1H").unwrap();

        display(&mut stdout,p.clone());
        draw_buttons(s,&mut stdout);

        stdout.flush().unwrap();

        if !input(&mut s, &mut p){break;}

    }
}

fn display<W: Write>(mut stdout: W,text:String){
    write!(stdout," ===========\n{ENTER}");
    write!(stdout," |");
    write!(stdout,"{text}");
    for i in 0..9-text.len() as usize{
        write!(stdout," ");
    }
    write!(stdout,"|");
    write!(stdout,"\n{ENTER}");
}

fn draw_buttons<W: Write>(s: i32, mut stdout: W){
    write!(stdout," ===========\n{ENTER}");
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
        write!(stdout,"|\n{ENTER}");
    }

    write!(stdout," ===========\n{ENTER}");
}

fn get_currect_symbol(x:i32) -> String{
    match x{
        1 => "1".to_string(),
        2 => "2".to_string(),
        3 => "3".to_string(),
        4 => "*".to_string(),
        5 => "4".to_string(),
        6 => "5".to_string(),
        7 => "6".to_string(),
        8 => "/".to_string(),
        9 => "7".to_string(),
        10 => "8".to_string(),
        11 => "9".to_string(),
        12 => "+".to_string(),
        13 => "x".to_string(),
        14 => "0".to_string(),
        15 => "=".to_string(),
        16 => "-".to_string(),
        _ => " ".to_string(),

    }
}

fn input(s:&mut i32,p:&mut String) -> bool{
    let stdin = io::stdin();
    if let Some(key) = stdin.keys().next() {
        match key.unwrap() {
            Key::Char('q') => false,
            Key::Down => {
                if (*s/3) < 4 || *s == 12{
                    *s += 4;
                }
                true
            },
            Key::Up => {
                if (*s/3) > 0 || (*s/3) > 1{
                    *s -= 4;
                }
                true
            },
            Key::Right => {
                if *s <= 15{
                    *s += 1;
                }
                true
            },
            Key::Left => {
                if *s >= 2{
                    *s -= 1;
                }
                true
            },
            Key::Char('\n') =>{
                let ch:String = get_currect_symbol(*s);
                if ch == "x".to_string(){
                    *p = String::from("");
                    return true;
                }
                if (ch == "*" || ch == "/" 
                || ch == "+" || ch == "-") 
                && p.ends_with(&ch) {
                    return true;
                }
                if (ch == "*" || ch == "/" 
                || ch == "+" || ch == "-") 
                && (p.len() >= 8 || p.len() == 0) {
                    return true;
                }
                if ch == "=".to_string(){
                    *p = eval(&*p).unwrap().to_string();
                    return true;
                }
                if p.len() < 9{
                    p.push_str(&ch);
                }
                true
            }
            _ => true,
        }
    }else{
        true
    }
}