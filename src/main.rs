use std::io;
fn main() {
    println!("please enter any num of your choice");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("invalid input");
    let temp = temp.trim().parse().expect("invalid input");
    println!("The prime numbers of {temp} are:");
    for i in 2..=temp {
        if i == 2 || i == 3 || i == 5 {
            print!("{},", i);
        } else if i % 2 == 1 && i % 3 >= 1 && i % 4 >= 1 && i % 5 >= 1 {
            print!("{},", i);
        }
    }
}
