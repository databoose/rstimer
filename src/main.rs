use std::io::{self, Write};
use std::{thread, time};
use std::error::Error;

use notify_rust::Notification;

fn printit(s: &str) {
	print!("{}", s);
	io::stdout().flush().unwrap();
}

fn get_pct_of(x:u16,y:u16) -> f32 {
	return x as f32 / y as f32 * 100.0;
}

fn get_secs_of(mut hour:u16, mut minute:u16, second:u16) -> u16 {
	let mut total_seconds = second;
	while hour > 0 {
		hour = hour - 1;
		total_seconds += 3600; // 3600 seconds in an hour
	}
	while minute > 0 {
		minute = minute - 1;
		total_seconds += 60;
	}
	return total_seconds;
}

fn main() -> Result<(), Box<dyn Error>>  {
    printit(&String::from("Enter time (h/m/s) : "));
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
    	Ok(_) => {
			let split = input.split(":");
			let vec = split.collect::<Vec<&str>>();

			assert_eq!(vec.len(), 3, "\n\n ERROR : User did not enter all three variables (h/m/s)\n");
			let mut hour = vec[0].trim().parse::<u16>().unwrap();
			let mut minute = vec[1].trim().parse::<u16>().unwrap();
			let mut second = vec[2].trim().parse::<u16>().unwrap();

			input = String::new(); // clear input
			let mut msg = String::new();
			printit("Enter message (enter if none) : ");
			match io::stdin().read_line(&mut input) {
				Ok(_) => {
					msg = String::from(input);
				}
				Err(error) => println!("error: {}", error),
			}

			let old_time = get_secs_of(hour, minute, second);
    		println!("Started timer...");
			loop  {
				if second <= 0 {
					if second <= 0 && minute <= 0 && hour <= 0 {
						println!("timer for [{}] done", msg);
						Notification::new()
							.summary("rstimer")
							.body(&msg)
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
				let current_time = get_secs_of(hour,minute,second);
				println!("{}:{}:{} ({:.2}% left) (Message : {})", hour, minute, second, get_pct_of(current_time, old_time), msg);
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
