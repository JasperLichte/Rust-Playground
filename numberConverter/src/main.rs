fn main() {
    let mut binary: i64 = 10101101101;
    print!("{}", binary);
    print!(" = ");
    let decimal: i64 = bin_to_dec(binary);
    print!("{}", decimal);
    print!("{}", " = ");
    binary = dec_to_bin(decimal);
    print!("{}", binary);
}

fn bin_to_dec(binary: i64) -> i64 {
    let mut res: i64 = 0;
    let str = binary.to_string();
    for (i, c) in str.chars().enumerate() {
        if c == '1' {
            res += i64::pow(2, i as u32);
        }
    }
    return res;
}

fn dec_to_bin(mut decimal: i64) -> i64 {
    let mut res: String = String::from("");
    while decimal > 0 {
        let part = decimal % 2;
        decimal = decimal / 2;
        let str: String = part.to_string();
        res.push_str(&str);
    }    
    return res.parse::<i64>().unwrap();
}
