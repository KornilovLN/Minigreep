//-----------------------------------------------------------------------------
//--- Программа тестирования и отработки методов создания и управления 
//--- чтением и записью текстовых файлов, поиском строк с указанием шаблона
//--- Используется библиотека lib.rs
//--- Разработан тест для проверки функции поиска search() в файле
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- github: "https://github.com/KornilovLN/Life.git"
//--- e-mail: KornilovStar@gmail.com
//--- e-mail: ln.starmark@ekatra.io
//--- e-mail: ln.starmark@gmail.com
//--- date:   11.08.2023 02:40:00
//-----------------------------------------------------------------------------

extern crate ansi_term;
use ansi_term::Colour;

//--- понадобится функция std::env::args из ст. библиотеки
use std::env;
//--- для выхода из программы с ненулевым сообщением системе
use std::process;

//--- библиотека lib.rs содержит: Config, run(); Секция Tests
use minigreep::{Config, About};


fn main() {

	//--- Получить и разместить в векторе аргументы строки запуска ПО
	let args: Vec<String> = env::args().collect();

	let about = About {
		firstname: "Leonid".to_string(),
		secondname: "Nikolaevich".to_string(),
		mainname: "Kornilov".to_string(),	
		author: "Kornilov LN (Starmark)".to_string(),
		github: "https://github.com/KornilovLN/Life.git".to_string(),
		e_mail: "ln.KornilovStar@gmail.com".to_string(),
		datetime: "11.08.2023 02:40:00".to_string(),
	};

	about.Out();
	about.Target();
	about.Waiter(1);

	//--- Попытка создать структуру данных на основе аргументов строки
	//--- и проанализировать на ошибки (по результату - продолжить или выйти)
	let config = Config::new(&args).unwrap_or_else(|err| {
		eprintln!("Проблема при разборе аргументов: {}", err);
		process::exit(1);
	});

	//--- вывести структуру config для убедительности
	config.out();

	//--- Основная работа ПО: поиск нужных строк по шаблону в файле 
	if let Err(e) = minigreep::run(config) {
		eprintln!("Ошибка в приложении: {}", e);
		


		process::exit(1);
	}	
	
}


