//--- Создали библиотеку для главного модуля main.rs
//--- переменные и методы должны быть pub

use std::error::Error;
use std::fs;

pub struct Config {
	pub prg: String,
	pub query: String,
	pub filename: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
	    if args.len() < 3 {
			return Err("недостаточно аргументов");
			
		}

		let prg = args[0].clone();
		let query = args[1].clone();
		let filename = args[2].clone();

		Ok(Config { prg, query, filename })
	}
}

impl Config {
	pub fn out(&self) {
		println!("Программа \t{}", self.prg);
		println!("Поиска  \t{}", self.query);
		println!("В файле \t{}", self.filename);
	}
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;
	
	println!("\nС текстом:\n{}", contents);

	Ok(())
}
