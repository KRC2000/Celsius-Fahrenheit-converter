use std::io;

pub fn read_line() -> String {
    let mut input = String::new();
    loop {
        input.clear();

        match io::stdin().read_line(&mut input) {
            Err(error) => {
                println!("Error: {error}.");
                continue;
            },
            Ok(_) => { return input; }
        }
    }
}

pub fn get_num() -> u8 {
    loop {
        let input = read_line();
        match input.trim().parse::<u8>() {
            Err(error) => { 
                println!("Error {error}. Input number."); 
                continue;
            },
            Ok(value) => { return value; }
        }
    }
}

pub fn get_float() -> f32 {
    loop {
        let input = read_line();
        match input.trim().parse::<f32>() {
            Err(error) => { 
                println!("Error {error}. Input number."); 
                continue;
            },
            Ok(value) => { return value; }
        }
    }
}