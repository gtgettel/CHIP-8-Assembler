use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::convert::AsRef;
use std::process;
use std::str;


fn exit_compilation(mut writer: &File){
	process::exit(1);
}


fn check_less_than_4096(mut int: &str, mut writer: &File) -> bool{
	int.to_string();
	let int: u16 = int.parse().unwrap();
	if int > 4095 {
		return false;
	}
	else {
		writer.write_fmt(format_args!("{:012b}", int));
		return true; 
	}
}


// TODO: track line number
fn parse_line(mut line: String, mut writer: &File, ln: i64){
	let mut line = str::replace(&line[..], ",", "");
	let mut line = line.split(" ");
	let words: Vec<&str> = line.collect();

	match &words[0] as &str{
		"SYS"  => {
			if words.len() > 2 {
				println!("Line {}: Too many arguments, SYS", ln);
				exit_compilation(writer);
			} else if words.len() <= 0 {
				println!("Line {}: Too few arguments, SYS", ln);
				exit_compilation(writer);
			}
			writer.write_fmt(format_args!("{:04b}", 0x0));
			if check_less_than_4096(&words[1] as &str, writer) {
				println!("Line {}: Argument outsize of address range, SYS takes 12-bit address", ln);
				exit_compilation(writer);
			}
			
		} 
		"CLS"  => {
			writer.write_fmt(format_args!("{:016b}", 0x00E0));
		}
		"RET"  => {
			writer.write_fmt(format_args!("{:016b}", 0x00EE));
		}
		"JP"   => {
			if words.len() > 3 {
				println!("Line {}: Too many arguments, JP", ln);
				exit_compilation(writer);
			} else if words.len() == 3 {
				if str::contains(&words[1],"V") {
					writer.write_fmt(format_args!("{:04b}", 0xB));
					if check_less_than_4096(&words[2] as &str, writer) {
						println!("Line {}: Argument outsize of address range, JP takes 12-bit address", ln);
						exit_compilation(writer);
					}
				} else {
					println!("Line {}: First argument is not register, JP", ln);
					exit_compilation(writer);
				}
			} else if words.len() == 2 {
				writer.write_fmt(format_args!("{:04b}", 0x1));
				if check_less_than_4096(&words[1] as &str, writer) {
					println!("Line {}: Argument outsize of address range, JP takes 12-bit address", ln);
					exit_compilation(writer);
				}
			} else {
				println!("Line {}: Too few arguments, JP", ln);
				exit_compilation(writer);
			}
		}
		// "CALL" => ,
		// "SE"   => ,
		// "SNE"  => ,
		// "ADD"  => ,
		// "OR"   => ,
		// "AND"  => ,
		// "XOR"  => ,
		// "SUB"  => ,
		// "SHR"  => ,
		// "SUBN" => ,
		// "SHL"  => ,
		// "RND"  => ,
		// "DRW"  => ,
		// "SKP"  => ,
		// "SKNP" => ,
		// "LD"   => ,
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
	let mut path = Path::new(&args[1]);
	let display = path.display();
	let mut file = match File::open(&path){
    Err(why) => panic!("Error: couldn't open {}", &display),
    Ok(file) => file,
  };
  // create output file
  let mut pathB = PathBuf::from(&args[1]); 
  pathB.set_extension("c8"); // change extension to chip-8 binary
  let displayB = pathB.display();
  let mut fileBinary = match File::create(&pathB){
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