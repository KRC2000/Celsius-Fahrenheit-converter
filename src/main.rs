use std::io;

fn main(){
    loop {
        println!("-------------------------------------------------------");
        println!("Choose converter:\n1) Celsius to Fahrenheit.\n2) Fahrenheit to Celsius.");
        
        
        loop {
            let input = get_num();

            if input == 1 {
                println!("How much Celsius?: ");
                let celsius = get_float();
                println!("That will be {} in Fahrenheit.", celsius * 1.8 + 32.0);
                return;
            }
            else if input == 2 {
                println!("How much Fahrenheit?: ");
                let farenheit = get_float();
                println!("That will be {} in Celsius.", (farenheit - 32.0) / 1.8 );
                return;
            }
            else {
                println!("No option {input}. Input number of converter you want to use.");             
            }
        }
    }
}


fn read_line() -> String {
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

fn get_num() -> u8 {
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

fn get_float() -> f32 {
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