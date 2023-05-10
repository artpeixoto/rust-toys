use std::{path::{PathBuf, Iter}, collections::HashMap, fs::{ReadDir, self}, default, str::Chars};

use web_server::tools::{basic_types::cached::Thunk, extensions::*};

pub struct Contentor<'a>(pub(in super) HashMap<String, Thunk<'a, String>>);
// ele gosta
// ratinho-o-o-o
impl<'a> Contentor<'a> {
	pub(in super) fn get_contentor_for_path(path: &PathBuf) -> Contentor<'a>{
        println!("Looking for content in {}", path.as_os_str().to_str().unwrap());
        fn worker(key_root :&str, d: &mut ReadDir, contentor: &mut Contentor) {
            for f in d.by_ref().filter_map(|x| {x.ok()}).map(|x| {x.path()}) {
                let current_file_name = f.file_name().unwrap().to_str().unwrap();
                if f.is_dir(){
                    let mut read_dir = f.read_dir().unwrap();
                    let next_root = String::new() + key_root  + current_file_name + "/";
                    worker(&next_root, &mut read_dir, contentor);
                } else if f.is_file() {
                    let file_getter = 
                        f.clone()
                        .used_in_once
                            (|file_copy: PathBuf| || {
                                println!("Reading {:?}", file_copy);
                                file_copy.used_in(&|x|{fs::read_to_string(x).unwrap()})
                            });

                    let mut key = key_root.to_string();

                    key.push_str(current_file_name);
                    contentor.0.insert(key, Thunk::new_getter( file_getter ));
                }
            }
        }
        
        let mut res = Contentor(HashMap::new());
        
        worker(
            "/", 
            &mut path.read_dir().unwrap(), 
            &mut res
            );

        res
    }

}