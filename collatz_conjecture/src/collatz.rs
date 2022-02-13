// A simple program that checks Collatz Conjecture for any (i32) number

pub fn run(mut n: i32) {
    let mut counter: i16 = 0;
    println!("{}: {}", counter, n);
    
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
            counter += 1;
            println!("{}: {}", counter, n);
        } else if n % 2 != 0 {
            n = n * 3 + 1;
            counter += 1;
            println!("{}: {}", counter, n);
        }
    }

    println!("TOTAL STEPS: {}", counter);
}
