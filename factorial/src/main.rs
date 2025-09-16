use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line( &mut input ).unwrap();

    let res = input.trim().parse::<u64>();

    match res {
        Ok(num) => println!("{}", num),
        Err(_) => println!("Not a valid number. "),
    }

}
