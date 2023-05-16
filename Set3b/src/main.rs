fn dodaj_pisemnie(a: String, b: String) -> String {
    let mut n1;
    let mut n2;
    let length;
    if a.len() >= b.len() {
        n1 = a.chars().rev();
        n2 = b.chars().rev();
        length = a.len();
    } else {
        n1 = b.chars().rev();
        n2 = a.chars().rev();
        length = b.len()
    }
    let mut result = String::new();
    let mut carry = 0;

    for i in 0..length {
        if n2.nth() == None {
            
        }
        let sum = n1.nth(i).unwrap() as u32 + n2.nth(i).unwrap() as u32 + carry;
        if sum > 9 {
            result.push_str((sum/10).to_string().as_str());
            carry = 1;
        } else {
            result.push_str(sum.to_string().as_str());
            carry = 0;
        }
    }

    result.chars().rev().collect()
}

fn main() {
    println!("{}", dodaj_pisemnie("123".to_string(), "123".to_string()));
}
