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
	
	//--- Создадим экземпляр Config и сохраним аргументы 
	//--- клонировав их к config полям структуры
	let config = Config::new(&args);
	config.out();
	
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

//--- Имплементация функции-конструктора для структуры Config
//--- Новый парсер вх аргументов заполняет структуру Config
//--- clone() необходим для полного копирования, а не только передачи ссылки
impl Config {
	fn new(args: &[String]) -> Config {
		let prg = args[0].clone();
		let query = args[1].clone();
		let filename = args[2].clone();
	
		Config { prg, query, filename }
	}
}

impl Config {
	//--- out() использует поля экземпляра своей структуры Config, 
	//--- поэтому пишем &self,
	//--- а знак ссылки & не дает завладеть экземпляром структуры 
	fn out(&self) {
		println!("Программа \t{}", self.prg);
		println!("Поиска  \t{}", self.query);
		println!("В файле \t{}", self.filename);
	}
}
