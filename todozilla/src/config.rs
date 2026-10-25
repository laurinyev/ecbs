use std::{fs, path::Path, collections::HashMap};

use serde::{Serialize, Deserialize};
use serde_yaml::{from_reader /*, to_writer */};

#[derive(Debug, Serialize, Deserialize)]
struct CommentFormat {
    ext: String,                 // File extension the comment format refers to
    nlt_start: Option<String>,   // Start pattern for a newline-terminated comment
    
    //The last two are coupled: both are needed if one is present
    block_start: Option<String>, // Start pattern for a block comment    
    block_end: Option<String>    // End pattern for a block comment
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Mandatory 
    comment_formats: Vec<CommentFormat>,          // List of project comment formats
    categories: Vec<String>,                      // List of category strings
    
    // Optinal
    domains: Option<Vec<HashMap<String, String>>>,// List of domains and their paths
    nesting_hierarchy: Option<String>,            // The hierarchy of headings in TZ.md
    base_dir: Option<String>,                     // The base directory to scan from
    md_path: Option<String>,                      // The path to the md file from base_dir
    markings_heading: Option<String>,             // The name of the Code Markings heading
    ephemeral_heading: Option<String>,            // The name of the Ephemeral TODOs heading 
}

impl Config {
    pub fn new(path: &Path) -> Result<Self, std::io::Error>{
        match from_reader(fs::File::open(path)?) {
            Ok(v) => Ok(v),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        } 
    }
    //TODO: writing
}
