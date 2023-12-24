use std::io;

fn main() {

    println!("Temp: ");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    println!("You input: {temp}");

    let temp: u32 = temp
        .trim()
        .parse()
        .expect("Please type a number");

    println!("Twice your temp is: {}", temp * 2);
}
