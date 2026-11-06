use std::{fs, path::Path, collections::HashMap};

use serde::{Serialize, Deserialize};
use serde_yaml::{from_reader, to_writer};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CommentFormat {
    pub ext: String,                 // File extension the comment format refers to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nlt_start: Option<String>,   // Start pattern for a newline-terminated comment
    
    //The last two are coupled: both are needed if one is present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_start: Option<String>, // Start pattern for a block comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_end: Option<String>    // End pattern for a block comment
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    // Mandatory 
    pub comment_formats: Vec<CommentFormat>,          // List of project comment formats
    pub categories: Vec<String>,                      // List of category strings
    
    // Optinal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<HashMap<String, String>>>,// List of domains and their paths
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_hierarchy: Option<String>,            // The hierarchy of headings in TZ.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_dir: Option<String>,                     // The base directory to scan from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub md_path: Option<String>,                      // The path to the md file from base_dir
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markings_heading: Option<String>,             // The name of the Code Markings heading
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_heading: Option<String>,            // The name of the Ephemeral TODOs heading 
}

impl Config {
    pub fn new(markings_heading: &Option<String>, ephemeral_heading: &Option<String>) -> Self{
        Self {
            comment_formats: vec![CommentFormat {
                ext: ".yml".to_string(),
                nlt_start: Some("#".to_string()),
                ..CommentFormat::default()
            }],
            categories: vec!["TODO".to_string(), "BUG".to_string(), "HACK".to_string()],
            markings_heading: markings_heading.clone(),
            ephemeral_heading: ephemeral_heading.clone(),
            ..Self::default()
        }
    }

    pub fn open(path: &Path) -> Result<Self, std::io::Error>{
        match from_reader(fs::File::open(path)?) {
            Ok(v) => Ok(v),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        } 
    }

    pub fn write(self: &Self, path: &Path) -> Result<(), std::io::Error> {
        match to_writer(fs::File::create(path)?, self) {
            Ok(v) => Ok(v),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        } 
    }

    //TODO: writing
}
