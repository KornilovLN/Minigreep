//--- нам понадобится функция std::env::args из стандартной библиотеки
use std::env;
//--- Нужно для выхода из программы с ненулевым сообщением системе
use std::process;

//--- наша библиотека
use minigreep::Config;


//--- Разбивка кода в библиотечную упаковку
fn main() {

	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	
	let config = Config::new(&args).unwrap_or_else(|err| {
		println!("Проблема при разборе аргументов: {}", err);
		process::exit(1);
	});
	config.out();
	
	if let Err(e) = minigreep::run(config) {
		println!("Ошибка в приложении: {}", e);

		process::exit(1);
	}	
	
}

/* Перенести в lib.rs
struct Config {
	prg: String,
	query: String,
	filename: String,
}

impl Config {
	fn new(args: &[String]) -> Result<Config, &'static str> {
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
	fn out(&self) {
		println!("Программа \t{}", self.prg);
		println!("Поиска  \t{}", self.query);
		println!("В файле \t{}", self.filename);
	}
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;
	
	println!("\nС текстом:\n{}", contents);

	Ok(())
}
*/
