use clap::{ App, Arg };
fn main() {
    let matches = App::new("temp_to_celsius")
        .arg(
            Arg::with_name("text")
                .value_name("Celsius")
                .help("Input text")
                .required(true)
                .min_values(1)
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let celsius: f64 = text[0].parse::<f32>().unwrap().into();
    let fahrenheit = (celsius*(9.0/5.0)) + 32.0;
    print!("{}° Celsius is equal to {}° Fahrenheit", celsius, fahrenheit)
}
