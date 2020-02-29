fn main() {
    println!("Hello, world!");
    let hello_world = String::from("Hello World!");
    let first_wrd = first_word(&hello_world);
    println!("{}", first_wrd);

    let s = String::from("hello");

    let len = s.len();

    let _slice = &s[0..len];
    let slice = &s[..];
    println!("{}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}