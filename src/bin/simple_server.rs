use web_server::internals::{extensions::usado_em::*, http_msgs::HttpReqBase, address, basic_types::{cached::*}};
use std::
    { fs::{self, ReadDir}
    , vec::Vec
    , io::
        { prelude::*
        , BufReader
        , BufWriter
        , Bytes, self
        , 
        }
    , net::
        { TcpListener
        , TcpStream, ToSocketAddrs
        }
    , thread::{self, current}
    , time::
        {   Instant
        ,   Duration
        }
    , ops::Add
    , array
    , fmt::Display, env::{args, self, split_paths}, collections::HashMap, path::{Path, PathBuf}, hash::Hash, str::FromStr
    };



struct Contentor<'a>(HashMap<String, Thunk<'a, String>>);


fn parse_path(path: &str) -> Vec<String> {
    let mut final_paths = Vec::<String>::new();
    let fourohfour = String::from("/404.html");
    
    if (path.ends_with("/")) {
        final_paths.push(String::new() + path + "index.html");
        final_paths.push(fourohfour)
    } else if let Ok(path_buf) = PathBuf::from_str(path) {
        if let Some(path_ext) = path_buf.extension(){
            if path_ext != "html"{
                let final_name  = path_buf.file_name().unwrap().to_str().unwrap();
                let ancestors       = path_buf.ancestors().skip(1);
                for ancestor in ancestors{
                    let ancestor_str = ancestor.to_str().unwrap().trim_end_matches('/');
                    final_paths.push(String::new() + ancestor_str + "/" + final_name);
                }
            } else {
                final_paths.push(path.to_string());
                final_paths.push(fourohfour)
            }
        } else {
            final_paths.push(String::new() + path + ".html");
            final_paths.push(String::new() + path + "/index.html");
            final_paths.push(fourohfour)
        }
    };
    println!("Procurando por {:?}", &final_paths);
    final_paths
}

fn get_contentor<'a>(web_server: &WebServerApplicationArgs) -> Contentor<'a>{
    println!("Looking for content in {}", web_server.content_root_dir.to_path_buf().as_os_str().to_str().unwrap());
    fn worker(key_root :&str, d: &mut ReadDir, contentor: &mut Contentor) {
        for f in d.by_ref().filter_map(|x| {x.ok()}).map(|x| {x.path()}){
            let current_file_name = f.file_name().unwrap().to_str().unwrap();
            if f.is_dir(){
                let mut read_dir = f.read_dir().unwrap();
                let next_root = String::new() + key_root  + current_file_name + "/";
                worker(&next_root, &mut read_dir, contentor);
            }
            else if f.is_file() {
                let file_copy = f.clone();
                let file_getter = move || {
                    println!("Reading {:?}", file_copy);
                    file_copy.used_in(&|x|{fs::read_to_string(x).unwrap()})
                };
                let mut key : String = key_root.to_string();
                key.push_str(current_file_name);
                println!("Adicionando \"{}\" -> \"{}\"", key, f.display());
                contentor.0.insert(key, Thunk::new_getter( file_getter));
            }
        }
    }
    let mut res = Contentor(HashMap::new());
    worker(
        "/", 
        &mut web_server.content_root_dir.read_dir().unwrap(), 
        &mut res
        );

    res
}


struct WebServerApplicationArgs{
    pub(crate) listen_port      : address::Address,
    pub(crate) content_root_dir : Box<Path>,
}



fn main() {
    let web_server_args = {     
        let cur_dir   = env::current_dir().unwrap();
        let endereco_ip 
            =  address::Address
                { ip:   (127, 0, 0, 1)
                , port: 7878 
                };
    
        WebServerApplicationArgs{
            listen_port:        endereco_ip,
            content_root_dir:   cur_dir.into_boxed_path(),
        }
    };
    
    let mut contentor = get_contentor(&web_server_args);
    
    let main_listener = 
        (&web_server_args)
            .listen_port
            .to_string()
            .used_in(&TcpListener::bind)
            .expect("Erro ao tentar acoplar-me ao endereço IP");
     
    println!("Executando leitor em {}", (&web_server_args).listen_port.to_string());      

    for mut stream in main_listener.incoming().filter_map(|x| {x.ok()}) {       
        
        let read_string =
                (&stream)
                .used_in(&BufReader::new)
                .lines()
                .map(Result::unwrap)
                .take_while(|l| {!l.is_empty()})
                .map(|x| {x + "\n"})
                .used_in(&String::from_iter);
        
        println!("Request received!");
        
        let read
            = match (&read_string).used_in(&|x| {HttpReqBase::try_parse(x)}) {
                Ok(req) => req,
                Err(_) => continue,
                };

        println!("Result gotten: \n\r\t{:?}\n\n", &read.path);
        
        
        let mut write_stream = 
                BufWriter::new(&mut stream);
        
        let full_response : String = {
            let no_content = String::new();
            let content = {
                let parsed_paths = parse_path(&read.path);
                let key_match = parsed_paths.iter().filter(|x| {(&contentor).0.contains_key(*x)}).next();
                match key_match{
                    Some(key) => {
                        println!("Found {}", key);
                        let res_thunk = contentor.0.get_mut(key).unwrap();
                        res_thunk.val_ref()
                    },
                    None => &no_content
                }
            };
            

            let content_length_header = format!("Content-Length: {}", content.len());
            let new_line = "\r\n";
            
            let res  = 
                (   [ "HTTP/1.1 200 OK"         , new_line
                    , &content_length_header    , new_line
                    , new_line
                    , content
                    ]
                ) .concat();

            res
        };
        
        write_stream
            .write_all(full_response.as_bytes())
            .unwrap();
        
     
        ()
    }
}
