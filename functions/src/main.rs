fn main() {
    println!("Hello, world!");

    another_function();

    function_parameter(11, 22);

    let x = multiplication(10, 11);
    println!("Result of multiplication: {}", x);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn another_function(){
    println!("Another function!");
}

fn function_parameter(x: i32, y: i32){
    println!("Value of x: {}", x);
    println!("Value of y: {}", y);
}

fn multiplication(a: i32, b: i32) -> i32 {
    //expression
    a * b

    //statement
    // a * b;
}