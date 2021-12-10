fn main(){
    println!("\n --- Temperature Conversion --- \n");

    let temperature_number = std::env::args().nth(1).expect("Please specify a number");
    let temperature_signal = std::env::args().nth(2).expect("Please, specify type");
    
    println!("Values entered: {:?} {:?}", temperature_number, temperature_signal);

    let temperature_number: f32 = temperature_number.trim().parse().expect("Not a Number");
    let mut temperature_signal: String = temperature_signal.to_string();


    let conversion: f32;
    
    if temperature_signal.to_lowercase() == "f"{

        conversion = (temperature_number - 32.0) / 1.8;
        temperature_signal = String::from("C");

        println!("\nTemperature: {}{}°", conversion, temperature_signal);

    }else if temperature_signal.to_lowercase() == "c"{
        conversion = (temperature_number * 1.8) + 32.0;
        temperature_signal = String::from("F");

        println!("\nTemperature: {}{}°", conversion, temperature_signal);
    }else{
        println!("\nThe signal entered not list in the options. Try 'F' or 'C'");
        conversion = 0.0;
        println!("\nTemperature: {} (Unable to convert)", conversion);
    }
    
    println!("\n --- Code Finished  --- \n");
}
