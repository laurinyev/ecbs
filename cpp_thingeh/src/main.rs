use std::{collections::HashMap, fs::{self, read_to_string}, path::{Path, PathBuf}};
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

#[derive(Clone)]
enum IncludeState {
    C, L, U, D, E, 
    WhiteSpacePre,
    PathAbsolute,
    PathRelative,
    WhiteSpacePost
}

#[derive(Clone)]
enum DefineState {
    E, F, I, N, E2,
    WhiteSpacePre,
    ContentStart,
    Content(String),
}

// IfNDef & IfDef state 
#[derive(Clone)]
enum IfDefState {
    D, E, F,
    WhiteSpacePre,
    PostIdent, //For deciding the state
    WhiteSpacePost,
}

#[derive(Clone)]
enum ParserState {
    StartOfLine,
    NonDirective,
    Identifier,
    Directive,
    DirectiveI,
    DirectiveIf,
    DirectiveIfDef(IfDefState),
    DirectiveIfNDef(IfDefState),
    DirectiveInclude(IncludeState),
    DirectiveE(bool), // false = first character, true = second character
    DirectiveElif,
    DirectiveElse,
    DirectiveEndif,
    DirectiveError,
    DirectiveDefine(DefineState),
    DirectiveUndef,
    DirectiveLine,
    DirectivePragma,
}

enum CommentState {
    NoComment,
    LineComment,
    BlockComment
}

#[derive(PartialEq)]
enum IfState {
    NotInIf,
    IfActive,
    ElseActive
}

fn process_file(contents: String, path: &Path, incl_search_paths: &[&Path], defines: &mut HashMap<String, String>) {
    let mut comment_state = CommentState::NoComment;
    let mut state = ParserState::StartOfLine;
    let mut do_forward = true;
    let mut working_string = String::new();
    let mut parent_state = ParserState::StartOfLine;
    let mut last_char = '\0';
    let mut if_state = IfState::NotInIf;
    let mut iter = contents.chars();
    let mut pushback: Option<char> = None;

    while let Some(c) = match pushback.take() { Some(v) => Some(v), None => iter.next()} {
        if last_char == '\\' && c == '\n' {
            last_char = c;
            continue;
        }

        match comment_state {
            CommentState::LineComment => {
                if c == '\n' {
                    comment_state = CommentState::NoComment;
                } else {
                    continue;
                }
            },
            CommentState::BlockComment => {
                if last_char == '*' && c == '/' {
                    last_char = ' ';
                    comment_state = CommentState::NoComment;
                    continue;
                }

                last_char = c;
                continue;
            },
            CommentState::NoComment => {
                if last_char == '/' {
                    match c {
                        '/' => {
                            print!(" ");
                            comment_state = CommentState::LineComment;
                            last_char = ' ';
                            continue;
                        },
                        '*' => {
                            print!(" ");
                            comment_state = CommentState::BlockComment;
                            last_char = ' ';
                            continue;
                        },
                        _ => print!("/")
                    }
                } else if c == '/' {
                    last_char = c;
                    continue;
                }
            }
        }

        match state {
            ParserState::StartOfLine => {
                if if_state == IfState::NotInIf { do_forward = true }
                match c {
                    '#' => state = ParserState::Directive,
                    '\n' => 
                        if do_forward {print!("{c}")}, // double newline is still start of line
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
            ParserState::Identifier => {
                match c {
                    _ if c.is_alphanumeric() || c == '_' => working_string.push(c),
                    _ => {
                        state = parent_state;
                        parent_state = ParserState::Identifier;
                        pushback = Some(c);
                    }
                }
            },
            ParserState::Directive => {
                match c {
                    ' ' | '\t' => (), // ISO C allows spaces and tabs between the # and the directive
                    'i' => state = ParserState::DirectiveI, 
                    'e' => state = ParserState::DirectiveE(false), 
                    'd' => state = ParserState::DirectiveDefine(DefineState::E), 
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
                    'd' => state = ParserState::DirectiveIfDef(IfDefState::E),
                    'n' => state = ParserState::DirectiveIfNDef(IfDefState::D),
                    _   => {
                        state = ParserState::NonDirective // TODO
                    }
                }
            },
            ParserState::DirectiveIfDef(ref s)=> {
                match (s, c) {
                    (IfDefState::E, 'e') => state = ParserState::DirectiveIfDef(IfDefState::F),
                    (IfDefState::F, 'f') => state = ParserState::DirectiveIfDef(IfDefState::WhiteSpacePre),
                   
                    // Whitespace
                    (IfDefState::WhiteSpacePre,  ' ')  => (),
                    (IfDefState::WhiteSpacePre,  '\t') => (),
                    (IfDefState::WhiteSpacePre,  _) => {
                        parent_state = ParserState::DirectiveIfDef(IfDefState::PostIdent);
                        state = ParserState::Identifier;
                        pushback = Some(c);
                    },

                    (IfDefState::WhiteSpacePost, ' ')  => (),
                    (IfDefState::WhiteSpacePost, '\t') => (),
                    (IfDefState::WhiteSpacePost, '\n') => state = ParserState::StartOfLine,

                    (IfDefState::PostIdent, c) => {
                        if defines.contains_key(&working_string) {
                            if_state = IfState::IfActive;
                        } else {
                            if_state = IfState::ElseActive;
                            do_forward = false;
                        }

                        match c {
                            '\n' => state = ParserState::StartOfLine,
                            ' ' | '\t' => state = ParserState::DirectiveIfDef(IfDefState::WhiteSpacePost),
                            _ => state = ParserState::NonDirective
                        }
                        working_string.clear();
                    }
                    
                    _ => state = ParserState::NonDirective
                }
            },
            ParserState::DirectiveIfNDef(ref s)=> {
                match (s, c) {
                    (IfDefState::D, 'd') => state = ParserState::DirectiveIfNDef(IfDefState::E),
                    (IfDefState::E, 'e') => state = ParserState::DirectiveIfNDef(IfDefState::F),
                    (IfDefState::F, 'f') => state = ParserState::DirectiveIfNDef(IfDefState::WhiteSpacePre),
                    
                    // Whitespace
                    (IfDefState::WhiteSpacePre,  ' ')  => (),
                    (IfDefState::WhiteSpacePre,  '\t') => (),
                    (IfDefState::WhiteSpacePre,  _) => {
                        parent_state = ParserState::DirectiveIfNDef(IfDefState::PostIdent);
                        state = ParserState::Identifier;
                        pushback = Some(c);
                    },

                    (IfDefState::WhiteSpacePost, ' ')  => (),
                    (IfDefState::WhiteSpacePost, '\t') => (),
                    (IfDefState::WhiteSpacePost, '\n') => state = ParserState::StartOfLine,

                    (IfDefState::PostIdent, c) => {
                        if !defines.contains_key(&working_string) {
                            if_state = IfState::IfActive;
                        } else {
                            if_state = IfState::ElseActive;
                            do_forward = false;
                        }

                        match c {
                            '\n' => state = ParserState::StartOfLine,
                            ' ' | '\t' => state = ParserState::DirectiveIfNDef(IfDefState::WhiteSpacePost),
                            _ => state = ParserState::NonDirective
                        }
                        working_string.clear();
                    }
                    
                    _ => state = ParserState::NonDirective
                }
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
                        let file = match search_incl_paths(&working_string, path, &incl_search_paths, false) {
                            Some(v) => v,
                            None    => {
                                eprintln!("Error: header <{working_string}> not found!");
                                return;
                            }
                        };
                        let contents = match read_to_string(&file) {
                            Ok(v)  => v,
                            Err(e) => {
                                eprintln!("Error: header <{working_string}> couldnt be read: {e}!");
                                return;
                            }
                        };
                        process_file(contents, &file, incl_search_paths, defines);
                        working_string.clear();
                        state = ParserState::DirectiveInclude(IncludeState::WhiteSpacePost);
                    },
                    (IncludeState::PathRelative, '"') => {
                        let file = match search_incl_paths(&working_string, path, &incl_search_paths, true) {
                            Some(v) => v,
                            None    => {
                                eprintln!("Error: header <{working_string}> not found!");
                                return;
                            }
                        };
                        let contents = match read_to_string(&file) { 
                            Ok(v)  => v,
                            Err(e) => {
                                eprintln!("Error: header <{working_string}> couldnt be read: {e}!");
                                return;
                            }
                        };
                        process_file(contents, &file, incl_search_paths, defines);
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
                if if_state == IfState::ElseActive {
                    do_forward = true;
                }
                state = ParserState::NonDirective;
            },
            ParserState::DirectiveEndif=> {
                if_state = IfState::NotInIf;
                do_forward = false;
                state = ParserState::NonDirective;
                
            },
            ParserState::DirectiveError=> {
                state = ParserState::NonDirective; // TODO
            },

            // Unambigupus ones: Define, Undef, Line & Pragma 

            ParserState::DirectiveDefine(ref def_state) => {
                match (def_state, c) {
                    (DefineState::E,  'e') => state = ParserState::DirectiveDefine(DefineState::F),
                    (DefineState::F,  'f') => state = ParserState::DirectiveDefine(DefineState::I),
                    (DefineState::I,  'i') => state = ParserState::DirectiveDefine(DefineState::N),
                    (DefineState::N,  'n') => state = ParserState::DirectiveDefine(DefineState::E2),
                    (DefineState::E2, 'e') => state = ParserState::DirectiveDefine(DefineState::WhiteSpacePre),

                    (DefineState::WhiteSpacePre, ' ')  => (), 
                    (DefineState::WhiteSpacePre, '\t') => (), 
                    (DefineState::WhiteSpacePre, _) => {
                        if c.is_alphabetic() {
                            parent_state = ParserState::DirectiveDefine(DefineState::ContentStart);
                            state = ParserState::Identifier;
                            pushback = Some(c);
                        } else {
                            return;
                        }
                    }, 

                    (DefineState::ContentStart, _) => {
                        if c == '\n' {
                            defines.insert(working_string.clone(), String::new());
                            working_string.clear();
                            state = ParserState::StartOfLine;
                        } else {
                            state = ParserState::DirectiveDefine(DefineState::Content(working_string));
                            working_string = String::new();
                            working_string.push(c);
                        }
                    }

                    (DefineState::Content(s), _) => {
                        if c == '\n' {
                            defines.insert(s.clone(), working_string.clone());
                            working_string.clear();
                            state = ParserState::StartOfLine;
                        } else {
                            working_string.push(c);
                        }
                    }

                    _ => state = ParserState::NonDirective
                }
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
        last_char = c;
    }

    if last_char != '\n' {
        print!("\n");
    }
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

    let mut defines = HashMap::<String, String>::new();

    process_file(contents, file, &incl_search_paths, &mut defines);
    
    println!("Defines: {defines:?}");
}
