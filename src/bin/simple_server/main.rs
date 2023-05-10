use std::{
    borrow::{BorrowMut},
    env::{self},
    io::{prelude::*, BufReader, BufWriter},
    net::{TcpListener},
    path::{PathBuf},
    str::FromStr,
    vec::Vec,
};

mod contentor;
use contentor::Contentor;
use web_server::{address, tools::extensions::UsadoEm, http_msgs::HttpReqBase};

fn parse_path(path: &str) -> Vec<String> {
    let mut final_paths = Vec::<String>::new();
    let fourohfour = String::from("/404.html");

    if path.ends_with("/") {
        final_paths.push(String::new() + path + "index.html");
        final_paths.push(fourohfour)
    } else if let Ok(path_buf) = PathBuf::from_str(path) {
        if let Some(path_ext) = path_buf.extension() {
            if path_ext != "html" {
                // É um recurso, como favicon.ico, ou algo assim.
                // Nesse caso, usamos outras possibilidades nos di          retorios superiores.
                let final_name = path_buf.file_name().unwrap().to_str().unwrap();
                let ancestors = path_buf.ancestors().skip(1);
                for ancestor in ancestors {
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

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

struct WebServerApplicationArgs {
    pub(crate) listen_port: address::Address,
    pub(crate) content_root_dir: PathBuf,
}

fn main() {
    let web_server_args = {
        let cur_dir = env::current_dir().unwrap();
        let endereco_ip = address::Address {
            ip:     (127, 0, 0, 1),
            port:   7878,
        };

        WebServerApplicationArgs {
            listen_port:        endereco_ip,
            content_root_dir:   cur_dir.to_path_buf(),
        }
    };

    let mut contentor = Contentor::get_contentor_for_path(&web_server_args.content_root_dir);

    let main_listener = 
        (&web_server_args)
        .listen_port
        .to_string()
        .used_in(&TcpListener::bind)
        .expect("Erro ao tentar acoplar-me ao endereço IP");

    println!(
        "Executando leitor em {}",
        (&web_server_args).listen_port.to_string()
    );

    for mut stream in main_listener.incoming().filter_map(|x| x.ok()) {
        let read_string = {
            
            let lines: Vec<String> = (&stream)
                .used_in(&BufReader::new)
                .lines()
                .map(Result::unwrap)
                .take_while(|l| !l.is_empty())
                .collect();

            let mut res = String::with_capacity(
                lines
                    .iter()
                    .map(|x| x.len())
                    .reduce(|acc, x| acc + 1 + x)
                    .unwrap(),
            );

            lines.iter().for_each(|x| {
                res.push_str(&x);
                res.push('\n')
            });

            res
        };

        println!("Request received!");

        let read = match (&read_string).used_in(&|x| HttpReqBase::try_parse(x)) {
            Ok(req) => req,
            Err(_) => continue,
        };

        println!("Result gotten: \n\r\t{:?}\n\n", &read.path);

        let mut write_stream = BufWriter::new(&mut stream);

        let full_response: String = {
            let contentor_mut_ref = &mut contentor;
            let possivel_content = {
                let parsed_paths = parse_path(&read.path);
                let key_match = parsed_paths
                    .iter()
                    .find(|x| contentor_mut_ref.0.contains_key(*x))
                    .and_then(|x| contentor_mut_ref.0.get_mut(x))
                    .map(|x| x.val_ref());
                key_match
            };

            let new_line = "\r\n";

            let res = match possivel_content {
                Some(content) => [
                    "HTTP/1.1 200 OK",
                    new_line,
                    &format!("Content-Length: {}", content.len()),
                    new_line,
                    new_line,
                    content,
                ]
                .concat(),
                None => ["HTTP/1.1 404 NOT FOUND", new_line, new_line].concat(),
            };

            println!("Enviando\n\n{}\n", &res);
            res
        };

        write_stream.write_all(full_response.as_bytes()).unwrap();
        ()
    }
}
