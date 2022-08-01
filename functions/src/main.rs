fn main() {
    println!("Hello, world!");
    another_function(7, 'a');
    let a = five();
    println!("five is equal to {a}");
    let b = plus_one(1);
    println!("b is equal to {b}");
}

fn another_function(x: i32, unit_label: char) {
    println!("Another function with param {x} with label {unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
