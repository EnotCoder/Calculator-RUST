
pub fn get_currect_mode(i:i32) -> String{
    match i{
        0 => " def".to_string(),
        1 => "exit".to_string(),
        _ => "....".to_string(),
    }
}

pub fn get_currect_symbol(x:i32) -> String{
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