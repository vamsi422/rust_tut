fn main() {
    println!("Hello, world!");
    let num = 11;
    let is_even = is_even(num);
    println!("{} is even: {}", num, is_even);
}

// function to check if a number is even

fn is_even(num:i32)->bool {
    return num % 2 == 0;
}