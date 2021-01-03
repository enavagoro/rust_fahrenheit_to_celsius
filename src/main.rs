use std::io;
use std::string::String;

fn main() {
    loop{
        let mut option = String::new();
        println!("Porfavor seleccione la sección que desea (F) Fahrenheit to Celsius (C) Celsius to Fahrenheit");

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option_uppercase = option.to_uppercase();

        let c = 'C'.to_string();
        let f = 'F'.to_string();

        if option_uppercase.trim() == c || option_uppercase.trim() == f{
            let mut temperature = String::new();

            println!("Porfavor Ingrese la temperatura");

            io::stdin()
                .read_line(&mut temperature)
                .expect("Failed to read line");

                let temperature: f64 = match temperature.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("Tu temperatura solo debe contener valores numericos");
                        continue;
                    }
                };

                println!("Temperatura ingresada: {}º{}", temperature,option_uppercase);
                if option_uppercase.trim() == c{
                    let final_temperature = celsius_to_fahrenheit(temperature);
                    println!("Temperatura Final: {}ºF", final_temperature);
                }
                else if option_uppercase.trim() == f{
                    let final_temperature = fahrenheit_to_celsius(temperature);
                    println!("Temperatura Final: {}ºC", final_temperature);
                }
        }
        else{
                println!("No has seleccionado una opción valida");
        }
    }
}

//T(°C) = (T(°F) - 32) × 5/9

fn fahrenheit_to_celsius(f_temp:f64) -> f64{
    (f_temp - 32.0) * (5.0/9.0)
}

//T(°F) = T(°C) × 9/5 + 32

fn celsius_to_fahrenheit(c_temp:f64) -> f64{
    (c_temp) * (9.0/5.0) + 32.0
}
