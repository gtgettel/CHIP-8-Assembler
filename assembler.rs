use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::convert::AsRef;
use std::process;


fn exit_compilation(mut writer: &File){
	process::exit(1);
}


fn parse_line(line: String, mut writer: &File){
	let mut line = line.split(" ");
	let words: Vec<&str> = line.collect();

	match &words[0] as &str{
		"SYS"  => {
			if words.len() > 2 {
				println!("Too many arguments: SYS addr");
				exit_compilation(writer);
			}
			let mut nnn = &words[1] as &str;
			nnn.to_string();
			let nnn_int: u16 = nnn.parse().unwrap();
			if nnn_int > 4095 {
				println!("Argument too large [addr]: SYS takes 12-bit address");
				exit_compilation(writer);
			}
			writer.write_fmt(format_args!("{:04b}", 0x0));
			writer.write_fmt(format_args!("{:012b}", nnn_int));
		} 
		"CLS"  => {
			writer.write_fmt(format_args!("{:016b}", 0x00E0));
		}
		"RET"  => {
			writer.write_fmt(format_args!("{:016b}", 0x00EE))
		}
		// "JP"   => ,
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
			println!("Unrecognized command: {}", &words[0]); // handle the rest of the cases
		}
	};
}


fn main(){
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
    parse_line(line.unwrap(), &fileBinary);
	}
}