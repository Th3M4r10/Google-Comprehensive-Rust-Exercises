fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

fn main() {
    println!("Length: {}", collatz_length(10));
}

fn main() {  
    let input = "a";
    match input.chars().next().unwrap() {
        'q' => println!("quit"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {}", key),
        key if key.is_uppercase() => println!("Uppercase: {}", key),
        _ => println!("Something else"),
    }
 }