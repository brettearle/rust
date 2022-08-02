fn main() {
    let fib = fibonacci(90);
    println!("Hello, Fibonacci! {fib}");
}

fn fibonacci(n: i32) -> u64 {
    if n < 0 {
        panic!("{} is negative!", n);
    } else if n == 0 {
        panic!("zero is not a right argument to fibonacci()!");
    } else if n == 1 {
        return 1;
    }
    let mut count: i32 = 1;
    loop {
        let mut sum = 0;
        let mut last = 0;
        let mut curr = 1;
        (1..n).for_each(|_i| {
            while count != n {
                count += 1;
                sum = last + curr;
                last = curr;
                curr = sum;
                println!("{count}  {sum}")
            }
        });
        if count == n {
            return sum;
        }
    }
}
