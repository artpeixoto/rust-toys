use std::env;

use clap::{self, command, Command, Arg, builder::{ValueParser, ValueParserFactory}};

use syn;

pub fn main() {
	let cmd = 
		Command::new("test")
		.arg(Arg::new("source").short('s').required(false).value_parser( ValueParser::path_buf()))	
		.arg(Arg::new("dest").short('d').required(false).value_parser(ValueParser::path_buf()));

    
	let matches = cmd.get_matches_from(env::args_os());

	println!("matches is {:?}", &matches);
}