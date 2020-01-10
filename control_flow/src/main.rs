fn main() {
    
    let number = 8;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("the number is divisible by 4")
    } else if number % 5 == 0 {
        println!("the number is divisible by 5");
    } else if number % 6 == 0 {
        println!("the number is divisible by 6");
    } else if number % 7 == 0 {
        println!("the number is divisible by 7");
    } else {
        println!("the number is only divisible by itself");
    }
}
