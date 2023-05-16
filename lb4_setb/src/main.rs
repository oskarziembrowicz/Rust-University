fn number_of_occurances(string: &str, sign: char) -> u32{
    let mut counter: u32 = 0;
    for i in string.chars(){
        if i == sign{
            counter += 1;
        }
    }
    counter
}

fn every_other_char(string: String, sign: char) -> String{
    let mut output: String = String::new();
    let mut counter = 0;
    for i in string.chars(){
        if counter % 2 == 0 {
            output.push(i);
        }
        counter += 1;
    }
    output
}

fn sypher(text: String, key: u32) -> String{
    let mut result = String::new();
    let mut tmp = String::new();
    let mut counter = 0;
    for i in text.chars(){
        counter +=1;
        tmp.push(i);
        if counter == key{
            for j in tmp.chars().rev(){
                result.push(j);
            }
            counter = 0;
            tmp = String::new();
        }
    }
    result.push_str(&tmp);
    return result;
}

fn card_name(f_name: String, l_name: String) -> String{
    let mut result = String::new();
    result.push(f_name.chars().nth(0).unwrap().to_ascii_uppercase());
    result.push_str(". ");
    let mut counter = 0;
    for i in l_name.to_ascii_lowercase().chars() {
        if(counter == 0){
            result.push(i.to_ascii_uppercase());
        }
        else {
            result.push(i);
        }
        counter += 1;
    }
    return result;
}

fn rom_to_arab(num: char) -> Option<u32> {
    let rom = ['I', 'V', 'X', 'L', 'D', 'C', 'M'];
    let arab = [1, 5, 10, 50, 100, 500, 1000];
    for i in 0..7 {
        if num == rom[i] {
            return Some(arab[i]);
        }
    }
    return None;
}

fn roman(text: String) -> u32 {
    let mut result= 0;
    let mut num1 = 10000;
    let mut num2 = 0;

    for i in text.chars() {
        num2 = rom_to_arab(i).unwrap();
        if num2 > num1 {
            result -= num1 * 2;
        }
        result += num2;
        num1 = num2;
    }
    return result;
}

fn to_roman(number: u32) -> String {
    let rom = vec1(
        (1000, "M"),
        (900, "DM"),
        (500, "C"),
        (400, "DC"),
        (100, "D"),
        (90, "XD"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I")
    );
    let mut result = String::new();
    let mut tmp = number;
    let mut i = 0;

    while tmp > 0{
        for e in rom{
            if e.0 <= tmp{
                result.push_str(e.1);
                tmp -= e.0;
                break;
            }
        }
    }
    result;
}

fn main() {
    let text = "analfabeta".to_string();
    // println!("{}", number_of_occurances(string.copy()));
    // println!("{}", every_other_char(string.to_string()));
    // println!("{}", sypher(text.clone(), 2));
    //println!("{}", card_name("Jamie".to_string(), "Stephenson".to_string()));
    println!("{}", roman("MMXII".to_string()));
}
