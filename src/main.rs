use std::fs;
use std::io;
use std::io::BufRead;
use std::env;
use std::process;
use std::collections::HashMap;
use std::error::Error;

mod city_generator;
mod errors;

struct Config {
    country: String,
    n: usize
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not enough arguments");
        }

        let country = args[1].clone();
        let n = match args[2].clone().parse::<usize>(){
            Ok(num) => num,
            Err(_) => 0,
        };

        if n <= 0 {
            return Err("Number of requested random cities must be greater than 0.")
        }

        Ok(Config { country, n })
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(config);


}


fn run(config: Config) -> Result<(), Box<dyn Error>>{

    // hardcoded settings for city generation for every country
    // country_code => (n_b, n_a, file path)
    let city_settings: HashMap<&str, (usize, usize, &str)> = HashMap::from([
        ("pl", (2, 1, "pl_cities.txt")),
        ("jp", (2, 2, "jp_cities.txt")),
        ("uk", (3, 3, "uk_cities.txt")),
        ("ua", (3, 1, "ua_cities.txt")),
    ]);

    let country = &config.country;
    if !city_settings.contains_key(country.as_str()) {
        panic!("Country code '{}' is not valid. Possible options: {:?}", country, city_settings.keys());
    }
    ;
    let (n_b, n_a, path) = city_settings[country.as_str()];
    let file =  match fs::File::open(path){
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e)
    };
    let reader = io::BufReader::new(file);

    let mut generator = crate::city_generator::CityGenerator::new(n_b, n_a);
    for line in reader.lines() {
        let name = match line {
            Ok(city) => city.to_lowercase(),
            Err(error) => panic!("Error: {:?}", error),
        };
        generator.add_word(&name);
    }
    for _ in 0..config.n{
        match generator.generate_random_word() {
            Ok(word) => println!("{}", word),
            Err(err) => eprintln!("ERROR: {:?}", err)
        }
    }
    Ok(())
}

