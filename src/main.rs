mod input;
mod temperature;

use temperature::Temperature;

fn main(){
    loop {
        println!("-------------------------------------------------------");
        println!("Choose converter:\n1) Celsius to Fahrenheit.\n2) Fahrenheit to Celsius.");
        
        
        loop {
            let input = input::get_num();

            if input == 1 {
                println!("How much Celsius?: ");
                let temp = Temperature::from_celsius(input::get_float());
                println!("That will be {} in Fahrenheit.", temp.get_fahrenheit());
                return;
            }
            else if input == 2 {
                println!("How much Fahrenheit?: ");
                let temp = Temperature::from_fahrenheit(input::get_float());
                println!("That will be {} in Celsius.", temp.get_celsius());
                return;
            }
            else {
                println!("No option {input}. Input number of converter you want to use.");             
            }
        }
    }
}
