use std::io;

fn main() {

    let mut found_num = false;
    let mut num : u64 = 0;

    while !found_num {
        println!("Input a number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let res = input.trim().parse::<u64>();

        match res {
            Ok(input_n) => {
                num = input_n;
                found_num = true;
            },
            Err(_) => {
                println!("Not a valid number. ")
            },
        }
    }

    let mut product : u64 = 1;

    for i in 1..=num {
        product *= i
    }

    println!("{}", product);

}
