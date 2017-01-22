use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::str;


/*
 * Exits on error. Closes file.
 */
fn exit_compilation(writer: &File){
	drop(writer); // close file
	process::exit(1);
}


/*
 * Checks range of 12-bit value. If okay, writes value to file.
 */
fn check_less_than_4096(call: &str, ln: i64, arg_num: i32, int: &str, mut writer: &File) -> () {
	int.to_string();
	let int: u16 = int.parse().unwrap(); // convert arg string to int
	if int > 4095 { // out of range
		println!("Line {0}: Argument {1} outsize of address range, {2} takes 12-bit address", ln, arg_num, call);
		exit_compilation(writer);
	} else {
		writer.write_fmt(format_args!("{:012b}", int));
	}
}


/*
 * Checks range of 8-bit value. If okay, writes value to file.
 */
fn check_less_than_256(call: &str, ln: i64, arg_num: i32, int: &str, mut writer: &File) -> () {
	int.to_string();
	let int: u8 = int.parse().unwrap(); // convert arg string to int
	if int > 255 { // out of range
		println!("Line {0}: Argument {1} outsize of range, {2} takes byte", ln, arg_num, call);
		exit_compilation(writer);
	} else {
		writer.write_fmt(format_args!("{:08b}",int));
	}
}


/*
 * Checks range of 4-bit value. If okay, writes value to file.
 */
fn check_less_than_16(call: &str, ln :i64, arg_num: i32, int: &str, mut writer: &File) -> () {
	int.to_string();
	let int: u8 = int.parse().unwrap(); // convert arg string to int
	if int > 15 { // out of range
		println!("Line {0}: Argument {1} outsize of range, {2}", ln, arg_num, call);
		exit_compilation(writer);
	} else {
		writer.write_fmt(format_args!("{:04b}",int));
	}
}


/*
 * Checks range of register argument. If okay, writes value to file.
 */
fn process_register_arg(call: &str, ln: i64, arg_num: i32, reg: &str, writer: &File) -> () {
	if str::contains(reg, "V") { // Is arg marked as register
		let reg = str::replace(reg, "V", ""); // remove V from arg
		check_less_than_16(call, ln, arg_num, &reg[..], writer); // check range
	}	else { // arg is not register
		println!("Line {0}: Argument {1} is not register", ln, arg_num);
		exit_compilation(writer);
	}
}


/*
 * Processes 8XYZ opcodes: AND, OR, XOR, ADD, SUB, SHR, SUBN, SHL
 */ 
fn process_8XYZ(call: &str, ln: i64, reg1: &str, reg2: &str, mut writer: &File, delim: u32, length: u32) -> () {
	if length > 3 {
		println!("Line {0}: Too many arguments, {1}", ln, call);
		exit_compilation(writer);
	} else if length < 3 {
		println!("Line {0}: Too few arguments, {1}", ln, call);
		exit_compilation(writer);
	} else {
		writer.write_fmt(format_args!("{:04b}", 0x8));
		// process register args
		process_register_arg(call, ln, 1, reg1, writer);
		process_register_arg(call, ln, 2, reg2, writer);
		writer.write_fmt(format_args!("{:04b}", delim));
	}
}


/*
 * Parses a line to binary
 */ 
fn parse_line(line: String, mut writer: &File, ln: i64){
	let line = str::replace(&line[..], ",", " "); // remove commas
	let line = line.split(" "); // split args on spaces
	let words: Vec<&str> = line.collect();

	match &words[0] as &str{
		"SYS"  => {
			if words.len() > 2 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() <= 0 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			}
			writer.write_fmt(format_args!("{:04b}", 0x0));
			check_less_than_4096(&words[0] as &str, ln, 1, &words[1] as &str, writer);		
		} 
		"CLS"  => {
			writer.write_fmt(format_args!("{:016b}", 0x00E0));
		}
		"RET"  => {
			writer.write_fmt(format_args!("{:016b}", 0x00EE));
		}
		"JP"   => {
			if words.len() > 3 {
				println!("Line {}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() == 3 { // check if "JP V0, NNN"
				if str::contains(&words[1],"V") { // is it a register
					writer.write_fmt(format_args!("{:04b}", 0xB));
					check_less_than_4096(&words[0] as &str, ln, 2, &words[2] as &str, writer);
				} else {
					println!("Line {0}: Argument {1} is not register, {2}", ln, 1, &words[0]);
					exit_compilation(writer);
				}
			} else if words.len() == 2 { // check if "JP NNN"
				writer.write_fmt(format_args!("{:04b}", 0x1));
				check_less_than_4096(&words[0], ln, 1, &words[1] as &str, writer);
			} else {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			}
		}
		"CALL" => {
			if words.len() > 2 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 2 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				writer.write_fmt(format_args!("{:04b}", 0x2));
				check_less_than_4096(&words[0], ln, 1, &words[1] as &str, writer);
			}
		}
		"SE"   => {
			if words.len() > 3 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 3 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				if str::contains(&words[2],"V") { // check if "SE Vx, Vy"
					writer.write_fmt(format_args!("{:04b}", 0x5));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					process_register_arg(&words[0] as &str, ln, 2, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:04b}", 0x0));
				} else { // else it is "SE Vx, NN"
					writer.write_fmt(format_args!("{:04b}", 0x3));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					check_less_than_256(&words[0] as &str, ln, 2, &words[2] as &str, writer);
				}
			}
		}
		"SNE"  => {
			if words.len() > 3 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 3 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				if str::contains(&words[2],"V") { // checks if "SNE Vx, Vy"
					writer.write_fmt(format_args!("{:04b}", 0x9));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					process_register_arg(&words[0] as &str, ln, 2, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:04b}", 0x0));
				} else { // else it is "SE Vx, NN"
					writer.write_fmt(format_args!("{:04b}", 0x4));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					check_less_than_256(&words[0] as &str, ln, 2, &words[2] as &str, writer);
				}
			}
		}
		"ADD"  => {
			if words.len() > 3 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 3 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				if &words[1] as &str == "I" { // checks if "ADD I, Vx"
					writer.write_fmt(format_args!("{:04b}", 0xF));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					writer.write_fmt(format_args!("{:08b}", 0x1E));
				} else if str::contains(&words[2],"V") { // else if "ADD Vx, Vy"
					writer.write_fmt(format_args!("{:04b}", 0x8));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					process_register_arg(&words[0] as &str, ln, 2, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:04b}", 0x4));
				} else { // else it is "ADD Vx, NN"
					writer.write_fmt(format_args!("{:04b}", 0x7));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					check_less_than_256(&words[0] as &str, ln, 2, &words[2] as &str, writer);
				}
			}
		}
		"OR"   => { // 8XY1
			process_8XYZ(&words[0] as &str, ln, &words[1] as &str, &words[2] as &str, writer, 0x1, words.len() as u32);
		}
		"AND"  => { // 8XY2
			process_8XYZ(&words[0] as &str, ln, &words[1] as &str, &words[2] as &str, writer, 0x2, words.len() as u32);
		}
		"XOR"  => { // 8XY3
			process_8XYZ(&words[0] as &str, ln, &words[1] as &str, &words[2] as &str, writer, 0x3, words.len() as u32);
		}
		"SUB"  => { // 8XY5
			process_8XYZ(&words[0] as &str, ln, &words[1] as &str, &words[2] as &str, writer, 0x5, words.len() as u32);
		}
		"SHR"  => {
			if words.len() > 2 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 2 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				writer.write_fmt(format_args!("{:04b}", 0x8));
				process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
				writer.write_fmt(format_args!("{:08b}", 0xF6));
			}
		}
		"SUBN" => { // 8XY7
			process_8XYZ(&words[0] as &str, ln, &words[1] as &str, &words[2] as &str, writer, 0x7, words.len() as u32);
		}
		"SHL"  => {
			if words.len() > 2 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 2 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				writer.write_fmt(format_args!("{:04b}", 0x8));
				process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
				writer.write_fmt(format_args!("{:08b}", 0xFE));
			}
		}
		"RND"  => {
			if words.len() > 3 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 3 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				writer.write_fmt(format_args!("{:04b}", 0xC));
				process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
				check_less_than_256(&words[0] as &str, ln, 2, &words[2] as &str, writer);
			}
		}
		"DRW"  => {
			if words.len() > 4 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 4 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				writer.write_fmt(format_args!("{:04b}", 0xD));
				process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
				process_register_arg(&words[0] as &str, ln, 2, &words[2] as &str, writer);
				check_less_than_16(&words[0] as &str, ln, 3, &words[3] as &str, writer);
			}
		}
		"SKP"  => {
			if words.len() > 2 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 2 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				writer.write_fmt(format_args!("{:04b}", 0xE));
				process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
				writer.write_fmt(format_args!("{:08b}", 0x9E));
			}
		}
		"SKNP" => {
			if words.len() > 2 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 2 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				writer.write_fmt(format_args!("{:04b}", 0xE));
				process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
				writer.write_fmt(format_args!("{:08b}", 0xA1));
			}
		}
		 "LD"   => {
		 	if words.len() > 3 {
				println!("Line {0}: Too many arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else if words.len() < 3 {
				println!("Line {0}: Too few arguments, {1}", ln, &words[0]);
				exit_compilation(writer);
			} else {
				if &words[1] as &str == "I" { // check if "LD I, _"
					if str::contains(&words[2],"V") { // check if "LD I, Vx"
						writer.write_fmt(format_args!("{:04b}", 0xF));
						process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
						writer.write_fmt(format_args!("{:08b}", 0x55));
					} else { // else it is "LD I, NNN"
						writer.write_fmt(format_args!("{:04b}", 0xA));
						check_less_than_4096(&words[0], ln, 1, &words[2] as &str, writer);
					}
				} else if &words[1] as &str == "DT" { // check if "LD DT, Vx"
					writer.write_fmt(format_args!("{:04b}", 0xF));
					process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:08b}", 0x15));
				} else if &words[1] as &str == "ST" { // check if "LD ST, Vx"
					writer.write_fmt(format_args!("{:04b}", 0xF));
					process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:08b}", 0x18));
				} else if &words[1] as &str == "F" { // check if "LD F, Vx"
					writer.write_fmt(format_args!("{:04b}", 0xF));
					process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:08b}", 0x29));
				} else if &words[1] as &str == "B" { // check if "LD B, Vx"
					writer.write_fmt(format_args!("{:04b}", 0xF));
					process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:08b}", 0x33));
				} else if str::contains(&words[1],"V") { // check if "LD Vx, _"
					if str::contains(&words[2],"V") { // check if "LD Vx, Vy"
						writer.write_fmt(format_args!("{:04b}", 0x8));
						process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
						process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
						writer.write_fmt(format_args!("{:04b}", 0x0));
					} else if &words[2] as &str == "DT" { // check if "LD Vx, DT"
						writer.write_fmt(format_args!("{:04b}", 0xF));
						process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
						writer.write_fmt(format_args!("{:08b}", 0x07));
					} else if &words[2] as &str == "I" { // check if "LD Vx, I"
						writer.write_fmt(format_args!("{:04b}", 0xF));
						process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
						writer.write_fmt(format_args!("{:08b}", 0x65));
					} else if &words[2] as &str == "K" { // check if "LD Vx, K"
 						writer.write_fmt(format_args!("{:04b}", 0xF));
						process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
						writer.write_fmt(format_args!("{:08b}", 0x0A));
					} else { // else it is "LD Vx, NN" 
						writer.write_fmt(format_args!("{:04b}", 0x6));
						process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
						check_less_than_256(&words[0], ln, 1, &words[2] as &str, writer);
					}
				} else {
					// Unrecognized argument
					println!("Line {0}: Unrecognized {1} arg, {2}", ln, 1, &words[0]);
					exit_compilation(writer);
				}
			}
		}
		// Super Chip-8 Opcodes
		// "SCD"  => ,
		// "SCR"  => ,
		// "SCL"  => ,
		// "EXIT" => ,
		// "LOW"  => ,
		// "HIGH" => ,
		_      => {
			// Unrecognized function call
			println!("Line {1}: Unrecognized command: {0}", &words[0], ln); // handle the rest of the cases
			exit_compilation(writer);
		}
	};
}


/*
 *
 */
fn main(){
	let mut ln: i64 = 0;
	// parse command line
	let args: Vec<String> = env::args().collect();
	let path = Path::new(&args[1]);
	let display = path.display();
	let file = match File::open(&path){
    Err(why) => panic!("Error: couldn't open {}", &display),
    Ok(file) => file,
  };
  // create output file
  let mut pathB = PathBuf::from(&args[1]); 
  pathB.set_extension("c8"); // change extension to chip-8 binary
  let displayB = pathB.display();
  let fileBinary = match File::create(&pathB){
    Err(why) => panic!("Error: couldn't create {}", &displayB),
    Ok(fileBinary) => fileBinary,
  };

  // Read the input file line by line
  let file = BufReader::new(file);
	for line in file.lines() {
		ln += 1;
    parse_line(line.unwrap(), &fileBinary, ln);
	}
}