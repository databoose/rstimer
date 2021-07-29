use std::io::{self, Write};
use std::{thread, time};
use std::error::Error;

use notify_rust::Notification;

fn printit(s: &str) {
	print!("{}", s);
	io::stdout().flush().unwrap();
}

fn main() -> Result<(), Box<dyn Error>>  {
    printit(&String::from("Enter time (h/m/s): "));
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
    	Ok(_) => {
			let split = input.split(":");
			let vec = split.collect::<Vec<&str>>();

			assert_eq!(vec.len(), 3, "\n\n ERROR : User did not enter all three variables (h/m/s)\n");
			let mut hour = vec[0].trim().parse::<u8>().unwrap();
			let mut minute = vec[1].trim().parse::<u8>().unwrap();
			let mut second = vec[2].trim().parse::<u8>().unwrap();
			
    		println!("Started timer...");
			loop  {
				if second <= 0 {
					if second <= 0 && minute <= 0 && hour <= 0 {
						println!("done");
						Notification::new()
							.summary("rstimer")
							.body("Timer done...")
							.show()?;
						break;
					}
					else {
						if minute <= 0 && hour >= 1 {
							minute = 60;
							hour = hour - 1;
						}
						minute = minute - 1;
						second = 60;
					}
				}
				println!("{}:{}:{}", hour, minute, second);
				print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
				
				thread::sleep(time::Duration::from_secs(1));
				second = second - 1;
			}
    	},

    	Err(e) => {
    		println!("Error : {}", e);
    	}
    }
	Ok(())
}
