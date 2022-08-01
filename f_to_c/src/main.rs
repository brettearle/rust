use std::io;

fn main() {
    println!("Hello, celsius");
    println!("Please input fahrenheit");

    let mut fahrenheit = String::new();

    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("failed to read line");

    let fahrenheit: f32 = match fahrenheit.trim().parse() {
        Ok(num) => num,
        Err(error) => panic!("Problem with parsing: {:?}", error),
    };

    fahrenheit_to_celsius(fahrenheit);
}

fn fahrenheit_to_celsius(f: f32) {
    let celsius = (f - 32.0) * (5.0 / 9.0);
    println!("{f} fahrenheit in celsius is {celsius}")
}
