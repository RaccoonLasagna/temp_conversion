use clap::{ App, Arg };
fn main() {
    let matches = App::new("temp_to_celsius")
        .arg(
            Arg::with_name("text")
                .value_name("Fahrenheit")
                .help("Input text")
                .required(true)
                .min_values(1)
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let fahrenheit: f64 = text[0].parse::<f32>().unwrap().into();
    let celsius = (5.0/9.0)*(fahrenheit - 32.0);
    print!("{}° Fahrenheit is equal to {}° Celsius", fahrenheit, celsius)
}
