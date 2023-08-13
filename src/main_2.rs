//--- нам понадобится функция std::env::args из стандартной библиотеки
use std::env;
//--- Для работы с файлом
use std::fs;

fn main() {
	//--- вызываем функцию env::args и сразу же при­меняем функцию collect
	//--- и привязываем коллекцию к неизменяемой переменной args 
	//--- типа Vec<string> 
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	
	//--- Сохраним аргументы привязав их к Config полям
	let config = parse_config(&args);
	out_config(&config);
	
	let contents = fs::read_to_string(config.filename)
		.expect("Что-то пошло не так при чтении файла");
		
	println!("С текстом:\n{}", contents);	
	
}

//--- Решаем использовать структуру вместо кортежа
struct Config {
	prg: String,
	query: String,
	filename: String,
}

//--- Новый парсер вх аргументов заполняет структуру Config
//--- clone() необходим для полного копирования, а не только передачи ссылки
fn parse_config(args: &[String]) -> Config {
	let prg = args[0].clone();
	let query = args[1].clone();
	let filename = args[2].clone();
	
	Config { prg, query, filename }
}

fn out_config(cnf: &Config) {
	println!("Программа \t{}", cnf.prg);
	println!("Поиска  \t{}", cnf.query);
	println!("В файле \t{}", cnf.filename);
}

