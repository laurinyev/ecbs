use std::{fs::{self, read_to_string}, path::{Path, PathBuf}};
use clap::{Parser};

#[derive(Parser)]
struct CmdLine {
    file: String
}

fn search_incl_paths(name: &str, path: &Path, paths: &[&Path], allow_rel: bool) -> Option<PathBuf> {
    let rel_path = path.join(name);
    if allow_rel && rel_path.exists() {
        return Some(rel_path); 
    } else {
        for p in paths {
            let conpath = p.join(name);
            if conpath.exists() {
                return Some(conpath);
            }
        }
    }

    None
}
#[derive(Clone, Copy)]
enum IncludeState {
    C, L, U, D, E, 
    WhiteSpacePre,
    PathAbsolute,
    PathRelative,
    WhiteSpacePost
}

enum ParserState {
    StartOfLine,
    NonDirective,
    Directive,
    DirectiveI,
    DirectiveIf,
    DirectiveIfDef,
    DirectiveIfNDef,
    DirectiveInclude(IncludeState),
    DirectiveE(bool), // false = first character, true = second character
    DirectiveElif,
    DirectiveElse,
    DirectiveEndif,
    DirectiveError,
    DirectiveDefine,
    DirectiveUndef,
    DirectiveLine,
    DirectivePragma,
}

fn main() {
    let args = CmdLine::parse();
    let file = Path::new(&args.file);

    let contents = match fs::read_to_string(file) {
        Ok(v) => v,
        Err(e) => {
            println!("Error reading file: {e}");
            return;
        }
    };

    let incl_search_paths = [
        Path::new("."),
        Path::new("include")
    ];

    let mut state = ParserState::StartOfLine;
    let do_forward = true;
    let mut working_string = String::new();

    for c in contents.chars() {
        match state {
            ParserState::StartOfLine => {
                match c {
                    '#' => state = ParserState::Directive,
                    _   => {
                        if do_forward { print!("{c}") };
                        state = ParserState::NonDirective
                    }
                }
            },
            ParserState::NonDirective => {
                if do_forward { print!("{c}") };
                if c == '\n' { 
                    state = ParserState::StartOfLine;
                }
            },
            ParserState::Directive => {
                match c {
                    ' ' | '\t' => (), // ISO C allows spaces and tabs between the # and the directive
                    'i' => state = ParserState::DirectiveI, 
                    'e' => state = ParserState::DirectiveE(false), 
                    'd' => state = ParserState::DirectiveDefine, 
                    'u' => state = ParserState::DirectiveUndef, 
                    'l' => state = ParserState::DirectiveLine, 
                    'p' => state = ParserState::DirectivePragma,
                    _   => state = ParserState::NonDirective
                } 
            },
            
            // I Directives
            
            ParserState::DirectiveI => {
                match c {
                    'f' => state = ParserState::DirectiveIf,
                    'n' => state = ParserState::DirectiveInclude(IncludeState::C),
                    _   => {
                        state = ParserState::NonDirective
                    }
                }
            },
            ParserState::DirectiveIf=> {
                match c {
                    'd' => state = ParserState::DirectiveIfDef,
                    'n' => state = ParserState::DirectiveIfNDef,
                    _   => {
                        state = ParserState::NonDirective // TODO
                    }
                }
            },
            ParserState::DirectiveIfDef => {
                state = ParserState::NonDirective; // TODO
            },
            ParserState::DirectiveIfNDef => {
                state = ParserState::NonDirective; // TODO

            },
            ParserState::DirectiveInclude(ref inc_state) => {
                match (inc_state, c) {
                    (IncludeState::C, 'c') => state = ParserState::DirectiveInclude(IncludeState::L),
                    (IncludeState::L, 'l') => state = ParserState::DirectiveInclude(IncludeState::U),
                    (IncludeState::U, 'u') => state = ParserState::DirectiveInclude(IncludeState::D),
                    (IncludeState::D, 'd') => state = ParserState::DirectiveInclude(IncludeState::E),
                    (IncludeState::E, 'e') => state = ParserState::DirectiveInclude(IncludeState::WhiteSpacePre),
                  
                    

                    // White space
                    (IncludeState::WhiteSpacePre,  ' ')  => (),
                    (IncludeState::WhiteSpacePre,  '\t') => (),
                    (IncludeState::WhiteSpacePre,  '<')  => state = ParserState::DirectiveInclude(IncludeState::PathAbsolute),
                    (IncludeState::WhiteSpacePre,  '"')  => state = ParserState::DirectiveInclude(IncludeState::PathRelative),
                    
                    (IncludeState::WhiteSpacePost, ' ')  => (),
                    (IncludeState::WhiteSpacePost, '\t') => (),
                    (IncludeState::WhiteSpacePost, '\n') => state = ParserState::StartOfLine,

                    // The path     
                    
                    (IncludeState::PathAbsolute, '>') => {
                        let file = match search_incl_paths(&working_string, file, &incl_search_paths, false) {
                            Some(v) => v,
                            None    => {
                                eprintln!("Error: header <{working_string}> not found!");
                                return;
                            }
                        };
                        let contents = match read_to_string(file) { //TODO: recursively process
                                                                    //these files
                            Ok(v)  => v,
                            Err(e) => {
                                eprintln!("Error: header <{working_string}> couldnt be read: {e}!");
                                return;
                            }
                        };
                        println!("{contents}");
                        working_string.clear();
                        state = ParserState::DirectiveInclude(IncludeState::WhiteSpacePost);
                    },
                    (IncludeState::PathRelative, '"') => {
                        let file = match search_incl_paths(&working_string, file, &incl_search_paths, true) {
                            Some(v) => v,
                            None    => {
                                eprintln!("Error: header <{working_string}> not found!");
                                return;
                            }
                        };
                        let contents = match read_to_string(file) { //TODO: recursively process
                                                                    //these files
                            Ok(v)  => v,
                            Err(e) => {
                                eprintln!("Error: header <{working_string}> couldnt be read: {e}!");
                                return;
                            }
                        };
                        println!("{contents}");
                        working_string.clear();
                        state = ParserState::DirectiveInclude(IncludeState::WhiteSpacePost);
                    },
                    
                    (IncludeState::PathAbsolute, d) => working_string.push(d),  
                    (IncludeState::PathRelative, d) => working_string.push(d),
                
                    _ => state = ParserState::NonDirective
                }
            },

            // E Directives

            ParserState::DirectiveE(el) => {
                if !el {
                    match c {
                        'n' => state = ParserState::DirectiveEndif,
                        'r' => state = ParserState::DirectiveError,
                        'l' => state = ParserState::DirectiveE(true),
                        _   => state = ParserState::NonDirective,
                    }
                } else {
                    match c {
                        'i' => state = ParserState::DirectiveElif,
                        's' => state = ParserState::DirectiveElse,
                        _   => state = ParserState::NonDirective,
                    }
                }
            },
            ParserState::DirectiveElif => {
                state = ParserState::NonDirective; // TODO
            },
            ParserState::DirectiveElse => {
                state = ParserState::NonDirective; // TODO
            },
            ParserState::DirectiveEndif=> {
                state = ParserState::NonDirective; // TODO
            },
            ParserState::DirectiveError=> {
                state = ParserState::NonDirective; // TODO
            },

            // Unambigupus ones: Define, Undef, Line & Pragma 

            ParserState::DirectiveDefine => {
                state = ParserState::NonDirective; // TODO
            },

            ParserState::DirectiveUndef => {
                state = ParserState::NonDirective; // TODO
            },

            ParserState::DirectiveLine => {
                state = ParserState::NonDirective; // TODO
            },
            
            ParserState::DirectivePragma => {
                state = ParserState::NonDirective; // TODO
            },
        }
    }

}
