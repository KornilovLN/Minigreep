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
	
	//--- Сохраним аргументы привязав их к неизменяемым переменным
	/*
	let prg = &args[0];
	let query = &args[1];
	let filename = &args[2];

	println!("Программа \t{}", prg);
	println!("Поиска  \t{}", query);
	println!("В файле \t{}", filename);
	*/
	let (prg, query, filename) = parse_config(&args);
	out_config(prg, query, filename);
	
	
	let contents = fs::read_to_string(filename)
		.expect("Что-то пошло не так при чтении файла");
		
	println!("С текстом:\n{}", contents);	
	
}

//--- Можно оформить парсер вх аргументов в отдельной функции
fn parse_config(args: &[String]) -> (&str, &str, &str) {
	let prg = &args[0];
	let query = &args[1];
	let filename = &args[2];
	
	(prg, query, filename)
}

fn out_config(prg: &str, query: &str, filename: &str) {
	println!("Программа \t{}", prg);
	println!("Поиска  \t{}", query);
	println!("В файле \t{}", filename);
}
