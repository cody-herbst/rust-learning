use std::env;

enum Currency {
    USD,
    GBP,
    JPY,
    KRW
}

fn main() {
    println!("Hello, world!");

    let mut dir : Option<String> = Option::None;
    match env::current_dir() {
        Ok(path) => {
            let dir_string = path.into_os_string().into_string().unwrap();
            dir = Some(dir_string);
        },
        Err(error) => {}
    }

}
