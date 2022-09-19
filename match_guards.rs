#[allow(dead_code)]

enum Temperature {
    Celsius(i32),
    Farenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(30);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{} is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{} is below 30 Celsius", t),
        Temperature::Farenheit(t) if t > 86 => println!("{}F is above 86 Farenheit", t),
        Temperature::Farenheit(t) => println!("{}F is below 86 Farenheit", t),
    }
}
