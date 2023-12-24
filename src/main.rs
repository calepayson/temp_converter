use std::io;

fn main() {

    println!("Temp: ");

    let mut f_temp = String::new();

    io::stdin()
        .read_line(&mut f_temp)
        .expect("Failed to read line");

    let f_temp: i32 = f_temp
        .trim()
        .parse()
        .expect("Please type a number");

    let c_temp = f_to_c(f_temp);
    println!("That is {c_temp} degrees Celcius");
}

fn f_to_c(temp: i32) -> i32 {
    ((temp - 32) * 5) / 9
}
