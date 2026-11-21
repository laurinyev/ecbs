use crate::config::*;
use std::{collections::HashMap, io::ErrorKind::{InvalidFilename, InvalidInput}, path::Path, vec};
use walkdir::WalkDir;

pub struct Marking {
    pub domain: Option<String>,
    pub file: String,
    pub category: String,
    pub text: String
}

struct CommentFormat {ntl_start: Option<String>, block_start: Option<String>, block_end: Option<String>}

pub fn scan_file(_config: &Config, file: &Path, domain: Option<String>) -> std::io::Result<Vec<Marking>> {
    println!("Scan file called with: <config ommitted for brevity> {file:?} {domain:?}");
     
    let filename = match file.to_str() {
        Some(v) => v,
        None => {
            return Err(std::io::Error::new(InvalidFilename, "Could not convert filename into a string."))
        }
    };
    
    println!("TODO: parse {filename}");
    

    Ok(vec![])
}

pub fn discover_markings(config: Config) -> std::io::Result<Vec<Marking>> {
    let dir2walk = config.base_dir.clone().unwrap_or(".".to_string());

    let mut comment_formats = HashMap::<String, CommentFormat>::new();

    for f in &config.comment_formats {
        if f.ext.len() == 0 {
            continue
        }

        let ext = if f.ext.chars().nth(0).unwrap() == '.' {
            f.ext[1..].to_string()
        } else {
            f.ext.clone()
        };

        comment_formats.insert(ext, 
            CommentFormat {ntl_start: f.nlt_start.clone(), block_start: f.block_start.clone(), block_end: f.block_end.clone()});
    }
    
    let walker = WalkDir::new(&dir2walk);

    let mut toret = vec![];

    for e in walker {
        if let Ok(e) = e {
            if !e.file_type().is_file() {
                continue;
            }

            if let Some(x) = e.path().extension() {
                if let Some(s) = x.to_str() {
                    if comment_formats.contains_key(s) {
                        let stripped_path = match e.path().strip_prefix(&dir2walk) {
                            Ok(v) => v,
                            Err(e) => return Err(std::io::Error::new(InvalidInput, format!("Could not strip path: {e}")))
                        };

                        let domain = config.domains.as_ref().and_then(|d| {
                            d.iter().find_map(|f| {
                                let (k, v) = f.iter().next()?;
                                let prefix = v.as_str();
                                stripped_path.starts_with(prefix).then(|| k.to_string())
                            })
                        });
                       
                        let file_markings = scan_file(&config, stripped_path, domain)?;
                        toret.extend(file_markings);
                    }
                }
            }
        }
    }

    Ok(toret)
}

