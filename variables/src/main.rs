fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 5;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "    ";
    println!("The spaces:{}.", spaces);

    let spaces = spaces.len();
    println!("The spaces lenght: {}", spaces);

    let u_integer: u64 = 8589934592;

    // let u_integer = u_integer * 2g
    println!("Usigned int: {}", u_integer * 2);

    let float_val: f32 = 2.75;

    println!("Float value in rust: {}", float_val);

    let y = float_val * u_integer as f32;

    println!("multiply of 2 different variables: {}", y);

    let t = true;

    let _f: bool = false;

    println!("value t: {}", t);

    let _c = 'z';

    let _z = 'ℤ';

    let heart_eyed_cat = '😻';

    println!("heart eyed cat: {}", heart_eyed_cat);

    let tup = ('a', 32, 32.1);
    
    let (character, _integer, _floatval) = tup;

    println!("the value of character is : {}", character);

    println!("destructing pattern matching: {}", tup.1);

    let array_a = [1,2,3,4,5];

    println!("The value array a is: {}",array_a[0]);

    let array_b: [i32; 5] = [0,1,2,3,4];

    println!("The value of arrayb is: {}", array_b[1]);

    let array_c = [3;5];

    println!("The value of array c is: {}", array_c[2]);

    for element in array_c.iter() {
        println!("{}", element);
    }
}
