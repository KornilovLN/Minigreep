//-----------------------------------------------------------------------------
//--- Библиотека для главного модуля main.rs 
//--- переменные и методы должны быть pub
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- e-mail: KornilovStar@gmail.com
//--- e-mail: ln.starmark@ekatra.io
//--- e-mail: ln.starmark@gmail.com
//--- date:   11.08.2023 02:40:00
//-----------------------------------------------------------------------------
//--- Дополнено тестом и методами работы с аргументами строки нечувствительными
//--- к регистру
//-----------------------------------------------------------------------------

//--- для Проверки переменной среды с именем CASE_INSENSITIVE создадим ее в BASH 
//--- Чтобы создать новую переменную оболочки с именем NEW_VARIABLE и значением 1, 
//--- и затем сразу экспортировать ее введите:
//--- $ export NEW_VARIABLE=1
//--- И проверяем:
//--- $ printenv NEW_VARIABLE
//--- 1
//--- Итак: если NEW_VARIABLE=1 - поиск будет без учета регистра, 
//--- иначе - при NEW_VARIABLE=0 - с учетом
use std::env;

extern crate ansi_term;
use ansi_term::Colour;

use std::error::Error;
use std::fs;

//=== Секция Config ===========================================================

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
	    if args.len() < 3 {
			return Err("недостаточно аргументов");
			
		}

		let query = args[1].clone();
		let filename = args[2].clone();
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config { query, filename, case_sensitive })
	}
}

impl Config {
	pub fn out(&self) {
		println!("\n--------------------------------------------------------");
		println!("--- Поиск   \t{}",self.query);
		println!("--- В файле \t{}",self.filename);
		println!("--------------------------------------------------------\n");
	}
}

//=== Секция Run ==============================================================

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

	//--- Из текста читаем в contens
	let contents = fs::read_to_string(config.filename)?;
    
	let mut cnt = 0;
	let mut abscnt = 0;

	//--- разбиваем contens на абзацы substrings, удаляя тем самым "\n"
	let substrings: Vec<&str> = contents.split("\n").collect();

    for st in substrings {

		//--- разбиваем на строки по точкам, удаляя их "."
		let subst: Vec<&str> = st.split(".").collect();

    	for elem in subst {

			//--- наконец - ищем нужные цепочки (по шаблону config.query)
			if config.case_sensitive {
			    //--- поиск с учетом регистра (CASE_INSENSITIVE=0) 
				for line in search(&config.query, &elem) {
					println!("({} - {})-> {}", cnt, abscnt, line);
				
					//--- наращиваем число находок
					cnt += 1;
				}
				
			} else {
			    //--- поиск без учета регистра (CASE_INSENSITIVE=1) 
				for line in search_case_insensitive(&config.query, &elem) {
					println!("({} - {})-> {}", cnt, abscnt, line);
				
					//--- наращиваем число находок
					cnt += 1;
				}
				
			};
	
			//--- получаем адрес следующей находки по номерам строк
			abscnt += 1;
		}

	}

	println!("\nВсего найдено {} строк, содержащих запрос\n", cnt);

	Ok(())
	
}

///--- справка: разбивка по пробелу (на будущее)
///--- let substrings: Vec<&str> = my_string.split_whitespace().collect();

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	
	//--- берем каждую строку из contens и 
	for line in contents.lines() {
	    //--- если она удовлетворяет запросу, то
		if line.contains(query) {
			//--- заносим ее в результаты
			results.push(line);
		}
	}
	
	results

}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();
	
	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}
	
	results
}


//=== Секция Tests ============================================================

//--- test разрабатываемой функции поиска строк по указанному содержимому -----
//--- и test для поиска строк чувствительных и нечувствительно к регистру -----

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
		
		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}
	
	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
	
		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	
	}
	
}	

	
		
		
