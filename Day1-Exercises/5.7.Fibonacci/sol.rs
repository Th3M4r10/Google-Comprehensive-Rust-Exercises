fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 20; // input your number
    println!("fib(n) = {}", fib(n));  // The 20th Fibonacci number is 6765 in 20 steps
}

