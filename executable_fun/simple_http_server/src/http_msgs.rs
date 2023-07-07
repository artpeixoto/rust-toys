
use std::
	{ fmt::
		{ Debug, Display
		}
	, collections::{
		HashMap
		}, error::Error
	, result::Result::*
};

use fun_tools::extensions::*;

type HeaderValContentType = String;


#[derive(Debug, Hash, PartialEq, Eq)]
pub struct HeaderName ( String );

#[derive(Debug)]
pub enum HeaderVal {
	Value(HeaderValContentType), 
	Flag()
}
#[derive(Debug)]
enum HeaderCollectionEntry {
	FlagEntry(HeaderName),
	ValueEntry(HeaderName, HeaderValContentType) 
}

impl From<Option<HeaderValContentType>> for HeaderVal{
    fn from(value: Option<HeaderValContentType>) -> Self {
        match value {
			Option::None => Self::Flag(),
			Option::Some(x) => Self::Value(x)
		}
    }




}
fn test(){
	let mut a = ((String::new(), String::new()),String::new());
	let ra = &mut a;
	{
		let a0 = &mut ra.0.0;
		
		fn test_ra0(fuck: &mut String){
			fuck.push_str( "asdf");
		}
		fn test_a0(mut fuck: String){
			fuck.push_str("rarara");
		}
		test_ra0(a0);
	};

	print!("{}", a.0.0);
}


impl Into<Option<HeaderValContentType>> for HeaderVal{
    fn into(self) -> Option<HeaderValContentType> {
        match self{
			HeaderVal::Flag()   => Option::None,
			HeaderVal::Value(x) => Option::Some(x),
		}
    }
}
impl HeaderVal{
	fn into(self) -> Option<HeaderValContentType> { 
		<HeaderVal as Into<Option<HeaderValContentType>>>::into(self) 
	}
}


impl HeaderCollectionEntry {
	fn new_flag(header_name: String) -> Self{
		HeaderCollectionEntry::FlagEntry(HeaderName(header_name))
	}
	fn new_value(header_name: String, header_val: String ) -> Self{
		HeaderCollectionEntry::ValueEntry(HeaderName(header_name), header_val)
	}
	
}

impl From<HeaderName> for HeaderCollectionEntry{
    fn from(value: HeaderName) -> Self {
        HeaderCollectionEntry::FlagEntry(value)
    }
}



impl From<(HeaderName, HeaderVal)> for HeaderCollectionEntry{
    fn from( booboo : (HeaderName, HeaderVal) ) -> Self {
        HeaderCollectionEntry::ValueEntry(booboo.0, booboo.1.into().unwrap())
    }
}



#[derive(Debug)]
pub struct HeaderCollection(HashMap<HeaderName, HeaderVal>);

impl HeaderCollection{
	fn new() -> HeaderCollection{
		HeaderCollection(HashMap::new())
	}
}

impl From<(Vec<HeaderName>, Vec<(HeaderName, HeaderValContentType)>)> for HeaderCollection {
	fn from(bobo: (Vec<HeaderName>, Vec<(HeaderName, HeaderValContentType)>)) -> Self{ 
		let (flags, values) = bobo;

		let mut content: HashMap<HeaderName, HeaderVal> = HashMap::new();

		for f in flags.into_iter(){
			content.insert(f, HeaderVal::Flag());
		}

		for (h, v) in values.into_iter(){
			content.insert(h, HeaderVal::Value(v));
		}

		HeaderCollection(content)
	}
}

impl From<Vec<HeaderCollectionEntry>> for HeaderCollection{
    fn from(value: Vec<HeaderCollectionEntry>) -> Self {
		let mut res = HeaderCollection::new();
		let HeaderCollection(res_fuck) = &mut res; 
		
        for entry in value.into_iter(){
			match entry{
				HeaderCollectionEntry::FlagEntry(header_name) 
					=> {res_fuck.insert(header_name, HeaderVal::Flag());},
				HeaderCollectionEntry::ValueEntry(header_name, header_val ) 
					=> {res_fuck.insert(header_name, HeaderVal::Value(header_val));},
			}
		}
		res
    }
}

#[derive(Debug)]
pub struct HttpReqBase{
	pub msg_type: String,
    pub command:  String,
	pub path: 	  String,
	pub headers:  HeaderCollection,
	pub content:  Option<String>
}

pub struct HttpRespBase{
    pub msg_type: 				String,
	pub return_code:  			u16,
	pub return_justification: 	String, 
	pub headers:  				HeaderCollection,
	pub content:  				Option<String>
}


impl HttpRespBase{
	
}

impl ToString for HttpRespBase{
    fn to_string(&self) -> String {
        todo!();
    }
}


#[derive(Debug)]
pub struct HttpParseError{
	pub what_happened : String
}

impl HttpParseError{
	fn new(what_happened: String) -> Self{
		HttpParseError { what_happened: what_happened }
	}
}

impl Display for HttpParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("fuck")
    }
}

impl Error for HttpParseError {
    
}

fn split_once_string<'a> (s: &'a str, delimiter: &'a str) -> (&'a str, Option<&'a str>){
	let del_size = delimiter.chars().count();
	for (index, _c) in s.char_indices(){
		if s[index..].starts_with(delimiter){
			return (&s[0..index], Some(&s[(index+del_size)..]));
		}
	}
	return (s, None);
}

impl HttpReqBase{
    pub fn try_parse(s: &str) -> Result<Self, HttpParseError> {
        let mut lines = s.lines();
		let (command, path, msg_type) = ({
			let mut rest = lines.next();
			let mut take_next_word 
				= || {
					let cur_string = rest.ok_or(HttpParseError::new("String is very empty".to_string()))?;
					let (res, new_rest) = match cur_string.split_once(' ') {
						Some((word, _rest)) => (word, Some(_rest)),
						None 				=> (cur_string, None) ,
					};
					rest = new_rest;
					Ok(res)
				};

			let command  = take_next_word()?.to_string();
			let path     = take_next_word()?.to_string();
			let msg_type = take_next_word()?.to_string();

			Ok((command, path, msg_type))
		})?;

		let headers = {
			let mut headers_vec: Vec<HeaderCollectionEntry> = Vec::new();

			while let Some(line) = lines.next().filter(|x| {!x.is_empty()}) { 	
				let (header_name, header_val) = line.used_in(&|x| {split_once_string(x, ": ")});
				headers_vec.push( 
					if header_val.is_none() 
						{ HeaderCollectionEntry::FlagEntry
							( HeaderName(header_name.to_string())
							) 
						}
					else 	
						{ HeaderCollectionEntry::ValueEntry
							( HeaderName(header_name.to_string())
							, header_val.unwrap().to_string()
							)
						}
				);
			}
			HeaderCollection::from(headers_vec)
		};
		
		let content = { 
			let mut content_string = String::new();
			
			while let Some(line) = lines.next().filter(|x| {!x.is_empty()}) {
				content_string.push_str(line);
			}

			if content_string.is_empty() {None} else {Some(content_string)}
		};

		Ok(HttpReqBase {  msg_type ,command, path, headers, content })
    }
} 

