fn main(){
    let number = 3;

    if number < 5 {
        println!("condition was true")
    } else {
        println!("condition was false")
    }

    if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 3 or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is {}", number);

    // loop {
    //     println!("Again!")
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        // thread::sleep(Duration::from_millis(1000));
        number -= 1;
    }

    println!("LIFTOFF!!");

    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5{
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let temperature = 50.0;
    println!("Degrees in fahrenheit: {}", temperature);

    let temperature = converter(temperature);
    println!("Degrees in celcius: {}", temperature);

    fib(128);
}

fn converter(fahrenheit: f32) -> f32{
    return(fahrenheit - 32.0) * 5.0/9.0
}

fn fib(long: u64) {
    let mut counter = 0;

    let _first = 0;
    let mut second = 0;
    let mut third = 0;

    if long == 0{
        print!("{}", counter);
        return;
    }
        

    while counter != long {
        print!("{} ", third);


        if third == 0 {
            third += 1;
            print!("{} ", third);
        }

        let _first = second;
        second = third;
        third = _first + second;

        counter += 1;
    };
    println!("");
}
