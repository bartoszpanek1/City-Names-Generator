use std::fs;
use std::io;
use std::io::BufRead;

mod city_generator;
mod errors;

fn main() {
    let file_name = "jp_cities.txt";

    let file = match fs::File::open(file_name) {
        Ok(file) => file,
        Err(error) => panic!("File could not be opened: {:?}", error),
    };
    let reader = io::BufReader::new(file);

    let mut chain = crate::city_generator::CityGenerator::new(2, 2);
    for line in reader.lines() {
        let name = match line {
            Ok(city) => city.to_lowercase(),
            Err(error) => panic!("Error: {:?}", error),
        };
        chain.add_word(&name);
    }
    for _ in 0..20 {
        let w = chain.generate_random_word(10);
        println!("{}", w.unwrap());
    }
}
