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

use std::error::Error;
use std::fs;

//=== Секция Config ===========================================================

pub struct Config {
	pub query: String,
	pub filename: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
	    if args.len() < 3 {
			return Err("недостаточно аргументов");
			
		}

		let query = args[1].clone();
		let filename = args[2].clone();

		Ok(Config { query, filename })
	}
}

impl Config {
	pub fn out(&self) {
		println!("\n--------------------------------------------------------");
		println!("--- Поиск   \t{}", self.query);
		println!("--- В файле \t{}", self.filename);
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
			for line in search(&config.query, &elem) {
				println!("({} - {})-> {}", cnt, abscnt, line);
				
				//--- наращиваем число находок
				cnt += 1;
			}
	
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

//=== Секция Tests ============================================================

//--- test разрабатываемой функции поиска строк по указанному содержимому -----

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
	    //--- будем искать строки с таким содержимым (менять и тестировать..)
		let query = "duct";
		//--- В этих 3-х строках
		let contents = "Rust:\nsafe, fast, productive.\nPick three.";

		assert_eq!(
		    //--- А это сам тест. В векторе требуемый результат,
		    //--- с которым должен совпасть результат поиска search()
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}
}	

	
		
		
