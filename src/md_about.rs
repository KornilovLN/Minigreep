//-----------------------------------------------------------------------------
//--- Модуль About 
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- Github: https://github.com/KornilovLN
//--- e-mail: ln.KornilovStar@gmail.com
//--- e-mail: ln.starmark@ekatra.io
//--- e-mail: ln.starmark@gmail.com
//--- date:   14.08.2023 13:10:00
//-----------------------------------------------------------------------------
//--- 
//--- 
//-----------------------------------------------------------------------------

extern crate datetime;
use std::{thread, time};
use chrono::Utc;

extern crate ansi_term;
use ansi_term::Colour;


#[derive(Debug)]
pub struct StAbout {
	firstname: String,	//--- имя
	secondname: String,	//--- отчество
	mainname: String,	//--- фамилия
	author: String,		//--- полный идентификатор автора
	github: String,		//--- Github 
	e_mail: String,		//--- почтовый ящик
	datetime: String,	//--- 14.08.2023 13:10:00
}

impl StAbout {
    pub fn new(
		firstn: &'static str, 
		secondn: &'static str, 
		mainn: &'static str,
		auth: &'static str, 
		gith: &'static str,
		mail: &'static str,
		dttm: &'static str,
		) -> StAbout {
			  Self{ firstname: firstn.to_string(), 
			  		secondname: secondn.to_string(), 
					mainname: mainn.to_string(),
					author: auth.to_string(), 
					github: gith.to_string(),
					e_mail: mail.to_string(),
					datetime: dttm.to_string(),
			  }  
	}
}

impl StAbout {
	pub fn waiter(&self, pause: u64) {
		thread::sleep(time::Duration::from_secs(pause));
	}
}


impl StAbout {
	pub fn get_datetime(&self) -> i64 {
		let dt = Utc::now();
		let timestamp: i64 = dt.timestamp_micros();	//timestamp(); - в секундах
		println!("{} {}",Colour::Blue.paint("Current timestamp is "), Colour::Blue.paint(timestamp.to_string()));
		
		timestamp		
	}
}

impl StAbout {
	pub fn out(&self) {
	    Self::target(self);
	    println!("\t--- About ---------------------------------------------------------------");
	    println!("{}",Colour::Green.paint(Self::tostring(&self)));
		println!("\t-------------------------------------------------------------------------\n");
	}
	
	pub fn tostring(&self) -> String {

	    let mut s = String::with_capacity(256);
	    //s.push_str("Structure StAbout");
	    
	    s.push_str("\tAuthor:      ");
	    s.push_str(&self.author);
	    s.push_str("\n\tFirst name:  ");
	    s.push_str(&self.firstname);
	    s.push_str("\n\tSecond name: ");
	    s.push_str(&self.secondname);

		s.push_str("\n\tMain name:   ");
	    s.push_str(&self.mainname);
	    s.push_str("\n\tGithub:      ");
	    s.push_str(&self.github);
	    s.push_str("\n\te-mail:      ");
	    s.push_str(&self.e_mail);
	    s.push_str("\n\tDate Time:   ");
	    s.push_str(&self.datetime);
		
	    s
	}
}

impl StAbout {
	pub fn target(&self) {
		println!("\t--- Программа -----------------------------------------------------------");
		println!("{}",Colour::Green.paint("\tПоиск в тексте по вхождению шаблона-строки\n"));
		println!("{}",Colour::Green.paint("\tтестирования методов создания и управления"));
		println!("{}",Colour::Green.paint("\tтекст-файлов, с поиском строк по шаблону"));
		println!("\t-------------------------------------------------------------------------");
	}
}
