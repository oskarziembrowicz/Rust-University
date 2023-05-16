use std::ffi::c_char;
use std::ops::Neg;
use std::str::Chars;

fn powtorki(v: &Vec<i32>) -> Vec<&i32> {
    let mut result = Vec::new();
    for i in 1..(v.len() - 1) {
        if v.get(i).unwrap() == v.get(i - 1).unwrap() ||
            v.get(i).unwrap() == v.get(i + 1).unwrap() {
            result.push(v.get(i).unwrap())
        }
    }

    result
}

fn unikalne(v: &Vec<i32>) -> Vec<&i32> {
    let mut result = Vec::new();
    for i in 0..v.len() {
        let mut unique = true;
        for j in 0..v.len() {
            if v.get(i).unwrap() == v.get(j).unwrap() && i != j{
                unique = false;
                break;
            }
        }
        if unique {
            result.push(v.get(i).unwrap());
        }
    }

    result
}

fn zlicz_wiele(s1: &Vec<i32>, s2: &Vec<i32>) -> u32 {
    let mut result = 0;
    for i in s1 {
        for j in s2 {
            if i == j {
            result += 1;
            }
        }
    }
    result
}

fn unikalne2(v: &Vec<i32>) -> Vec<&i32> {
    let mut result = Vec::new();
    for i in v {
        if zlicz_wiele(v, &vec![*i]) == 1{
            result.push(i);
        }
    }

    result
}

fn indeksy(v: &Vec<i32>, x: i32) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..v.len() {
        if *(v.get(i).unwrap()) == x {
            result.push(i);
        }
    }
    result
}

fn lowercaseVector() -> Vec<char> {
    let v = ('a'..='z').collect();
    v
}

fn squaresVector() -> Vec<i32> {
    let v = (1..=10).map(|x| x*x).collect();
    v
}

fn reverseNumbersVector() -> Vec<f64> {
    let v = (1..=20).map(|x| 1.0/(x as f64)).collect();
    v
}

fn deviableByThreeVector() -> Vec<i32> {
    let v = (1..=100).filter(|x| x%3 == 0 && x%4 != 0).collect();
    v
}

fn stringsShorterThanFour(v: &Vec<String>) -> Vec<&String> {
    let mut result = Vec::new();
    for i in v.iter() {
        if i.len() < 4 {
            result.push(i);
        }
    }
    result
}

fn stringsWithoutA(v: &Vec<String>) -> Vec<&String> {
    let mut result = Vec::new();
    for i in v.iter() {
        let mut noA = true;
        for j in i.chars() {
            if j == 'a' || j == 'A' {
                noA = false;
                break;
            }
        }
        if noA {
            result.push(i);
        }
    }
    result
}

fn stringsWithNumbers(v: &Vec<String>) -> Vec<&String> {
    let mut result = Vec::new();
    for i in v.iter() {
        let mut hasNumber = false;
        for j in i.chars() {
            if j.is_numeric() {
                hasNumber = true;
                break;
            }
        }
        if hasNumber {
            result.push(i);
        }
    }
    result
}

fn main() {
    println!("{:?}", powtorki(&vec![1,2,3,3,3,3,4,5,6,6,7]));
    println!("{:?}", unikalne(&vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6]));
    println!("{:?}", zlicz_wiele(&vec![1, 2, 1, 3], &vec![1, 2]));
    println!("{:?}", unikalne2(&vec![1, 3, 4, 3, 3, 5, 3, 4, 1, 1, 6]));
    println!("{:?}", indeksy(&vec![1,2,3,2,5,6,2,2], 2));

    println!("{:?}", lowercaseVector());
    println!("{:?}", squaresVector());
    println!("{:?}", reverseNumbersVector());
    println!("{:?}", deviableByThreeVector());

    println!("{:?}", stringsShorterThanFour(&vec!["abc".to_string(), "abab".to_string(), "ugabuga".to_string(), "pum".to_string()]));
    println!("{:?}", stringsWithoutA(&vec!["abc".to_string(), "abab".to_string(), "ugabuga".to_string(), "pum".to_string()]));
    println!("{:?}", stringsWithNumbers(&vec!["ab1".to_string(), "abab".to_string(), "u24".to_string(), "pum".to_string()]));
}
