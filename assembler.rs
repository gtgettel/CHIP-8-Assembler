use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::str;


fn exit_compilation(writer: &File){
	process::exit(1);
}


fn check_less_than_4096(call: &str, ln: i64, arg_num: i32, int: &str, mut writer: &File) -> () {
	int.to_string();
	let int: u16 = int.parse().unwrap();
	if int > 4095 {
		println!("Line {0}: Argument {1} outsize of address range, {2} takes 12-bit address", ln, arg_num, call);
		exit_compilation(writer);
	} else {
		writer.write_fmt(format_args!("{:012b}", int));
	}
}


fn check_less_than_256(call: &str, ln: i64, arg_num: i32, int: &str, mut writer: &File) -> () {
	int.to_string();
	let int: u8 = int.parse().unwrap();
	if int > 255 {
		println!("Line {0}: Argument {1} outsize of range, {2} takes byte", ln, arg_num, call);
		exit_compilation(writer);
	} else {
		writer.write_fmt(format_args!("{:08b}",int));
	}
}


fn check_less_than_16(call: &str, ln :i64, arg_num: i32, int: &str, mut writer: &File) -> () {
	int.to_string();
	let int: u8 = int.parse().unwrap();
	if int > 15 {
		println!("Line {0}: Argument {1} outsize of range, {2}", ln, arg_num, call);
		exit_compilation(writer);
	} else {
		writer.write_fmt(format_args!("{:04b}",int));
	}
}


fn process_register_arg(call: &str, ln: i64, arg_num: i32, reg: &str, writer: &File) -> () {
	if str::contains(reg, "V") {
		let reg = str::replace(reg, "V", "");
		check_less_than_16(call, ln, arg_num, &reg[..], writer);
	}	else {
		println!("Line {0}: Argument {1} is not register", ln, arg_num);
	}
}


fn process_8XYZ(call: &str, ln: i64, reg1: &str, reg2: &str, mut writer: &File, delim: u32, length: u32) -> () {
	if length > 3 {
		println!("Line {0}: Too many arguments, {1}", ln, call);
		exit_compilation(writer);
	} else if length < 3 {
		println!("Line {0}: Too few arguments, {1}", ln, call);
		exit_compilation(writer);
	} else {
		writer.write_fmt(format_args!("{:04b}", 0x8));
		process_register_arg(call, ln, 1, reg1, writer);
		process_register_arg(call, ln, 2, reg2, writer);
		writer.write_fmt(format_args!("{:04b}", delim));
	}
}


// TODO: track line number
fn parse_line(line: String, mut writer: &File, ln: i64){
	let line = str::replace(&line[..], ",", "");
	let line = line.split(" ");
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
			} else if words.len() == 3 {
				if str::contains(&words[1],"V") {
					writer.write_fmt(format_args!("{:04b}", 0xB));
					check_less_than_4096(&words[0] as &str, ln, 2, &words[2] as &str, writer);
				} else {
					println!("Line {0}: Argument {1} is not register, {2}", ln, 1, &words[0]);
					exit_compilation(writer);
				}
			} else if words.len() == 2 {
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
				if str::contains(&words[2],"V") {
					writer.write_fmt(format_args!("{:04b}", 0x5));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					process_register_arg(&words[0] as &str, ln, 2, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:04b}", 0x0));
				} else {
					writer.write_fmt(format_args!("{:04b}", 0x3));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					check_less_than_16(&words[0] as &str, ln, 2, &words[2] as &str, writer);
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
				if str::contains(&words[2],"V") {
					writer.write_fmt(format_args!("{:04b}", 0x9));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					process_register_arg(&words[0] as &str, ln, 2, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:04b}", 0x0));
				} else {
					writer.write_fmt(format_args!("{:04b}", 0x4));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					check_less_than_16(&words[0] as &str, ln, 2, &words[2] as &str, writer);
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
				if &words[2] as &str == "I" {
					writer.write_fmt(format_args!("{:04b}", 0xF));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					writer.write_fmt(format_args!("{:08b}", 0x1E));
				} else if str::contains(&words[2],"V") {
					writer.write_fmt(format_args!("{:04b}", 0x8));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					process_register_arg(&words[0] as &str, ln, 2, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:04b}", 0x4));
				} else {
					writer.write_fmt(format_args!("{:04b}", 0x7));
					process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
					check_less_than_16(&words[0] as &str, ln, 2, &words[2] as &str, writer);
				}
			}
		}
		"OR"   => {
			process_8XYZ(&words[0] as &str, ln, &words[1] as &str, &words[2] as &str, writer, 0x1, words.len() as u32);
		}
		"AND"  => {
			process_8XYZ(&words[0] as &str, ln, &words[1] as &str, &words[2] as &str, writer, 0x2, words.len() as u32);
		}
		"XOR"  => {
			process_8XYZ(&words[0] as &str, ln, &words[1] as &str, &words[2] as &str, writer, 0x3, words.len() as u32);
		}
		"SUB"  => {
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
		"SUBN" => {
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
				if &words[1] as &str == "I" {
					if str::contains(&words[2],"V") {
						writer.write_fmt(format_args!("{:04b}", 0xF));
						process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
						writer.write_fmt(format_args!("{:08b}", 0x55));
					} else {
						writer.write_fmt(format_args!("{:04b}", 0xA));
						check_less_than_4096(&words[0], ln, 1, &words[2] as &str, writer);
					}
				} else if &words[1] as &str == "DT" {
					writer.write_fmt(format_args!("{:04b}", 0xF));
					process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:08b}", 0x15));
				} else if &words[1] as &str == "ST" {
					writer.write_fmt(format_args!("{:04b}", 0xF));
					process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:08b}", 0x18));
				} else if &words[1] as &str == "F" { 
					writer.write_fmt(format_args!("{:04b}", 0xF));
					process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:08b}", 0x29));
				} else if &words[1] as &str == "B" {
					writer.write_fmt(format_args!("{:04b}", 0xF));
					process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
					writer.write_fmt(format_args!("{:08b}", 0x33));
				} else if str::contains(&words[1],"V") {
					if str::contains(&words[2],"V") {
						writer.write_fmt(format_args!("{:04b}", 0x8));
						process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
						process_register_arg(&words[0], ln, 1, &words[2] as &str, writer);
						writer.write_fmt(format_args!("{:04b}", 0x0));
					} else if &words[2] as &str == "DT" {
						writer.write_fmt(format_args!("{:04b}", 0xF));
						process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
						writer.write_fmt(format_args!("{:08b}", 0x07));
					} else if &words[2] as &str == "I" {
						writer.write_fmt(format_args!("{:04b}", 0xF));
						process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
						writer.write_fmt(format_args!("{:08b}", 0x65));
					} else if &words[2] as &str == "K" {
 						writer.write_fmt(format_args!("{:04b}", 0xF));
						process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
						writer.write_fmt(format_args!("{:08b}", 0x0A));
					} else {
						writer.write_fmt(format_args!("{:04b}", 0xA));
						process_register_arg(&words[0], ln, 1, &words[1] as &str, writer);
						check_less_than_4096(&words[0], ln, 1, &words[2] as &str, writer);
					}
				} else {
					println!("Line {0}: Unrecognized {1} arg, {2}", ln, 1, &words[0]);
				}
				writer.write_fmt(format_args!("{:04b}", 0xC));
				process_register_arg(&words[0] as &str, ln, 1, &words[1] as &str, writer);
				check_less_than_256(&words[0] as &str, ln, 2, &words[2] as &str, writer);
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
			println!("Line {1}: Unrecognized command: {0}", &words[0], ln); // handle the rest of the cases
		}
	};
}


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