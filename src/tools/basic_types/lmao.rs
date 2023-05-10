use std::{collections::HashMap, sync::{Arc, mpsc}, thread::{self, spawn}, io::{BufRead, self, Read}, usize, fs::{self, ReadDir}, env::current_dir, path::PathBuf, str::FromStr};

use crate::tools::extensions::UsadoEm;


type GigabyteMap = HashMap<String, String>;

#[test]
pub fn process_files_in_prallel_test(){
	let filenames = 
		PathBuf::from("C:\\Users\\artur\\Documents")
			.read_dir()
			.map(|x| 
				x
				.flatten()
				.map(|y| y.path().to_str().unwrap().to_string()) 
				.collect::<Vec<_>>()
				)
			.expect("Erro ao obter os arquivos")
			;
	
	let gloss = 
		[ ("in", "into")
		, ("as", "ass")
		, ("piece", "zein paice")
		, ("a", "one")
		, ("single", "singular")
		]
		.map(|par| (par.0.to_owned(), par.1.to_owned()))
		.used_in(&|x| HashMap::from(x)	)
		.used_in(&|x| Box::new(x)		);
	
	process_files_in_parallel_using_threads(&filenames, gloss)
}

pub fn process_files_in_parallel_using_pipes(filenames: &[String], glossario: Box<GigabyteMap>){
	let arc: Arc<GigabyteMap> = Arc::from(glossario);
	let (sender, receiver) = mpsc::channel::<(&str, Arc<GigabyteMap>)>();
	let res = match receiver.try_recv(){
		Ok(val) => Ok(Some(val)),
		Err(std::sync::mpsc::TryRecvError::Empty) => Ok(None),
		Err(_e) => Err(_e)
	};
}



pub fn process_files_in_parallel_using_threads(filenames: &[String], glossario: Box<GigabyteMap>){
	let arc: Arc<GigabyteMap> = Arc::from(glossario);

	let handles = 
		filenames
		.iter()
		.map(|x| (x.to_string(), arc.clone()))
		.map(|(filename, gloss)| {
			let new_thread = thread::spawn(move || {
					let mut file = fs::File::open(&filename)?;
					println!("lendo arquivo {filename}");
					
					process_file(file, &gloss)
				});

			println!("iniciando thread {:?}", (&new_thread).thread().id());

			new_thread
			})	
		.collect::<Vec<_>>();

	for handle in handles {
		let handle_id = handle.thread().id();
		let handle_res = handle.join();
		println!("Resultado de {:?} sucesso: 	 {:?}", handle_id, handle_res.is_ok());
	}
}	
pub fn process_file(mut file: impl Read, gloss: &GigabyteMap) -> io::Result<String>{

	let mut file_content: String = String::new();
	
	file.read_to_string(&mut file_content)?;

	println!("Filesize is {}", &file_content.len());
	let words 
		= file_content
			.split_whitespace()
			.map(|word:&str| gloss.get(word).map(|x| {println!("found {}", &x); &x[..]}).unwrap_or(word))
			.collect::<Vec<&str>>();
	
	let mut final_res = String::with_capacity(words.iter().map(|x| (*x).len() + 1).sum());
	
	for x in words{
		final_res.push_str(x);
	}
	//println!("{}", &final_res);
	Ok(final_res)
}

