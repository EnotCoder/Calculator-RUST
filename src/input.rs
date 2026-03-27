use std::io;
use termion::event::Key;//считывание с клавиратуры
use termion::input::TermRead;//Трейт для чтения событий клавиатуры
use evalexpr::*;

use crate::currect::*;
use crate::display::*;

pub fn input_calc(program_state:&mut i32,s:&mut i32,p:&mut String) -> bool{
    let stdin = io::stdin();
    if let Some(key) = stdin.keys().next() {
        match key.unwrap() {
            Key::Char('q') => false,
            Key::Down => {
                if *s < 13{
                    *s += 4;
                }
                true
            },
            Key::Up => {
                if *s > 4{
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
            Key::Ctrl('c') => {*program_state = -1;true},
            _ => true,
        }
    }else{
        true
    }
}

pub fn input_mode(program_state:&mut i32,m:&mut i32) -> bool{
    let stdin = io::stdin();
    if let Some(key) = stdin.keys().next() {
        match key.unwrap() {
            Key::Char('q') => false,
            Key::Down => {
                if *m != 1{
                    *m += 1;
                }
                true
            },
            Key::Up => {
                if *m != 0{
                    *m -= 1;
                }
                true
            },
            Key::Char('\n') => {
                *program_state = *m;
                true
            }
            _ => true,
        }
    }else{
        true
    }
}