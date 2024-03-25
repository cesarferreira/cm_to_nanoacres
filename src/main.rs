// fn main() {
//     println!("Hello, world!");
// }

use clap::{App, Arg};

fn main() {
    let matches = App::new("Square Centimeters to Nanoacres Converter")
        .version("1.0")
        .author("Cesar Ferreira <cesar.manuel.ferreira@gmail.com>")
        .about("Converts square centimeters to nanoacres")
        .arg(Arg::with_name("cm2")
             .help("The area in square centimeters to convert")
             .required(true)
             .index(1))
        .get_matches();

    let cm2: f64 = matches.value_of("cm2").unwrap().parse().expect("Please provide a valid number");

    let nanoacres = convert_cm2_to_nanoacres(cm2);
    println!("{} square centimeters is {} nanoacres", cm2, nanoacres);
}

fn convert_cm2_to_nanoacres(cm2: f64) -> f64 {
    (cm2 / 10_000.0) / 4_046.8564224 * 1_000_000_000.0
}
