fn delta(i: f64, j: f64) -> f64{
    if i == j {
        1.0
    } else {
        0.0
    }
}

// fn wymierna(x: i32) -> f64 {
//     if x==0 {
//         -1.0
//     } else {
//         1.0/(x.abs() as f64)
//     }
// }

fn wymierna(x: i32) -> Option<f64> {
    if x==0 {
        None
    } else {
        Some(1.0/(x.abs() as f64))
    }
}

fn has_special_char(s: String) -> bool {
    (s.contains('@') || s.contains('%') || s.contains('!'))
}

fn has_lowercase(s: String) -> bool {
    for i in s.chars() {
        if(i.is_ascii_lowercase()){
            return true;
        }
    }
    false
}

fn has_uppercase(s: String) -> bool {
    for i in s.chars() {
        if(i.is_ascii_uppercase()){
            return true;
        }
    }
    false
}

fn silne_haslo(s: String) -> bool {
    if(s.len() >= 8 && has_special_char(s.clone()) && has_lowercase(s.clone()) && has_uppercase(s.clone())){
        return true;
    }
    false
}

fn main() {
    println!("Delta: {}", delta(1.0,2.0));
    println!("Absolute: {}", wymierna(2).unwrap());
    println!("Is the password strong: {}", silne_haslo("M@rt1nH3id3gg3R".to_string()));
}
