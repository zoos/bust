use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let digit = args[1].parse::<u32>().unwrap();
    let base = &args[2];
    comb("", digit, base);
}

fn comb(begin: &str, digit: u32, base: &str) {
    for i in 0..base.len() {
        let mut v = String::from(begin);
        v.push(base.chars().nth(i).unwrap());

        if v.len() < digit as usize {
            comb(&v[..], digit, base);
        } else {
            println!("{}", v);
        }
    }
}
