fn main() {
    let n = 10;

    println!("the {n}th fibonacci number is {}", fibonacci(n));
}

fn fibonacci(n: i32) -> i32 {
    // Very slow function to get the nth fib number
    // Done this way to try ou recursion
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}


