use std::io;

pub fn input(text: &str) -> String {
    let mut inp = String::new();
    println!("{}", text);
    io::stdin()
        .read_line(&mut inp)
        .expect("ввел человек какую-то хрень, бывает...");
    return inp.trim().to_string();
}