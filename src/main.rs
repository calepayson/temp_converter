use std::io;

fn main() {
    println!("Please input a temperature: ");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    println!("You input: {temp}");

    // let temp: u32 = match temp.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };

    // println!("You")
}
