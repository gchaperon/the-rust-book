fn main() {
    let celsius = 15.7;
    let farenheit = 67.3;

    println!("{} degrees celsius is {} degrees farenheit", celsius, celsius_to_farenheit(celsius));
    println!("{} degrees farenheit is {} degrees celsius", farenheit, farenheit_to_celsius(farenheit));
}

fn celsius_to_farenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn farenheit_to_celsius(farenheit: f32) -> f32 {
    (farenheit - 32.0) * 5.0 / 9.0
}
