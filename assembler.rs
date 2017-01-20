use std::io::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::convert::AsRef;


fn parse_line(line: String, mut writer: &File){
	let mut line = line.split(" ");
	let words: Vec<&str> = line.collect();

	match &words[0] as &str{
		"SYS"  => {
			writer.write_fmt(format_args!("{:b}", 0));
		} 
		"CLS"  => {
			writer.write_fmt(format_args!("{:b}", 0x00E0));
		}
		// "RET"  => ,
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