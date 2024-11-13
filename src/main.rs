use core::prelude;

const PROGRAM_NAME: &str = "smartconf";

macro_rules! usage {
    ($program_name: expr) => {
        eprintln!("Usage:");
        eprintln!("{}: [OPTION]... FILE", $program_name);
        eprintln!("{}: FILE [OPTION]...", $program_name);
        eprintln!("{}: FILE [OPTION]... -- [ARGUMENT]...", $program_name);
    };
}

macro_rules! help {
    ($program_name: expr) => {
        usage!($program_name);
        todo!();
    };
}

#[allow(unused_macros)]
macro_rules! error {
    ($($msg: expr),+) => {
        eprintln!("error: {}", format!($($msg,)+));
    };
}

#[allow(unused_macros)]
macro_rules! warning {
    ($($msg: expr),+) => {
        eprintln!("warning: {}", format!($($msg,)+));
    };
}

struct Config {
    input_file_name: String,
    arguments: Vec<String>,
    starting_line_number: usize,
    starting_column_number: usize,
}

// cla = command-line arguments
fn parse_cla(program_name: String, mut args: ::std::env::Args) -> Config {
    #[allow(unused_macros)]
    macro_rules! cla_parser_error {
        ($($msg: expr),+) => {
            error!("cla parser: {}", format!($($msg,)+));
        };
    }

    #[allow(unused_macros)]
    macro_rules! cla_parser_warning {
        ($($msg: expr),+) => {
            warning!("cla parser: {}", format!($($msg,)+));
        };
    }

    let mut result = Config {
        input_file_name: String::from(""),
        arguments: Vec::new(),
        starting_line_number: 1,
        starting_column_number: 1,
    };
    enum State {
        #[allow(non_camel_case_types)] NONE,
        #[allow(non_camel_case_types)] OPTION_ARGUMENT{option: String},
    }
    let mut state = State::NONE;
    match args.next() {
        Some(arg) => {
            result.input_file_name = arg;
        }
        None => {
            error!("input file name not provided");
            ::std::process::exit(1);
        }
    }
    while args.len() != 0 {
        let arg = args.next().unwrap();
        match state {
            State::NONE => {
                enum State {
                    #[allow(non_camel_case_types)] NONE,
                    #[allow(non_camel_case_types)] ARGUMENT{argument: String},
                    #[allow(non_camel_case_types)] SHORT_OPTIONS{options: Vec<String>},
                    #[allow(non_camel_case_types)] LONG_OPTION{option: String},
                    #[allow(non_camel_case_types)] LONG_OPTION_WITH_OPTIONS{option: String, options: Vec<String>},
                }
                let mut state = State::NONE;
                while arg.len() != 0 {
                    let letter = arg.chars().next();
                    let letter = match letter {
                        Some(c) => c,
                        None => {
                            cla_parser_error!("Cannot get letter of a string");
                            ::std::process::exit(2);
                        }
                    };
                    match state {
                        State::NONE => {
                            if letter == '-' {
                                state = State::SHORT_OPTIONS { options: Vec::new() };
                                continue;
                            }
                            state = State::ARGUMENT { argument: String::from(letter) };
                        },
                        State::ARGUMENT { mut argument } => {
                            argument.push(letter);
                            state = State::ARGUMENT { argument }
                        }
                        State::SHORT_OPTIONS { ref mut options } => {
                            if letter == '-' {
                                state = State::LONG_OPTION_WITH_OPTIONS { option: String::new(), options: options.to_vec() };
                                continue;
                            }
                        },
                        State::LONG_OPTION { mut option } => {
                            option.push(letter);
                            state = State::LONG_OPTION { option };
                        },
                        State::LONG_OPTION_WITH_OPTIONS { mut option, options } => {
                            option.push(letter);
                            state = State::LONG_OPTION_WITH_OPTIONS { option, options }
                        }
                    }
                }
            },
            State::OPTION_ARGUMENT { option } => {
                todo!();
            },
        }
    }
    return result;
}

#[derive(Clone, Debug)]
struct Loc {
    filename: String,
    line_number: usize,
    column_number: usize,
}

#[derive(Debug)]
enum TokValue {
    #[allow(non_camel_case_types)] IDENTIFIER { value: String },
    #[allow(non_camel_case_types)] SPECCHAR { value: String },
    #[allow(non_camel_case_types)] STRING { value: String, quote_type: char },
    #[allow(non_camel_case_types)] ONE_LINE_COMMENT,
}

#[derive(Debug)]
struct Tok {
    loc: Loc,
    value: TokValue,
}

fn lex(program_name: String, config: Config, input: String) -> Vec<Tok> {
    #[allow(unused_macros)]
    macro_rules! lexer_error {
        ($($msg: expr),+) => {
            error!("lexer: {}", format!($($msg,)+));
        };
    }

    #[allow(unused_macros)]
    macro_rules! lexer_loc_error {
        ($loc: expr, $($msg: expr),+) => {
            lexer_error!("{}: {}: {}: {}", $loc.filename, $loc.line_number, $loc.column_number, format!($($msg,)+));
        };
    }

    #[allow(unused_macros)]
    macro_rules! lexer_warning {
        ($($msg: expr),+) => {
            warning!("lexer: {}", format!($($msg,)+));
        };
    }

    #[allow(unused_macros)]
    macro_rules! lexer_loc_warning {
        ($loc: expr, $($msg: expr),+) => {
            lexer_warning!("{}: {}: {}: {}", $loc.filename, $loc.line_number, $loc.column_number, format!($($msg,)+));
        };
    }

    #[derive(Debug)]
    enum State {
        IDENTIFIER,
        SPECCHAR,
        STRING { quote_type: char, escaping: bool },
        NOP,
        WHITESPACE,
        ONELINECOMMENT,
    }
    let mut state = State::IDENTIFIER;
    let mut result = Vec::new();
    let mut loc = Loc {
        filename: config.input_file_name.clone(),
        line_number: config.starting_line_number,
        column_number: config.starting_column_number - 1,
    };
    let mut prev_loc = Loc {
        filename: config.input_file_name,
        line_number: config.starting_line_number,
        column_number: config.starting_column_number,
    };
    let mut current_text = String::new();
    macro_rules! add_tok {
        () => {
            match state {
                State::NOP => {
                    None
                },
                State::ONELINECOMMENT => {
                    None
                },
                _ => {
                    let value = match state {
                        State::SPECCHAR => {
                            TokValue::SPECCHAR {
                                value: current_text.clone(),
                            }
                        },
                        State::IDENTIFIER => {
                            TokValue::IDENTIFIER {
                                value: current_text.clone(),
                            }
                        },
                        State::STRING { quote_type, escaping } => {
                            TokValue::STRING {
                                value: current_text.clone(),
                                quote_type,
                            }
                        },
                        _ => unreachable!(),
                    };
                    Some(Tok {
                        loc: prev_loc.clone(),
                        value,
                    })
                },
            }
        };
    }
    for c in input.chars() {
        match state {
            State::STRING { quote_type, escaping } => {
                if escaping {
                    let c = match c {
                        '\\' => '\\',
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '0' => '\0',
                        'a' => '\x07',
                        'b' => '\x08',
                        'v' => '\x0B',
                        'f' => '\x0C',
                        '\'' => '\'',
                        '"' => '"',
                        _ => {
                            lexer_loc_error!(loc, "Wrong escaping character");
                            ::std::process::exit(3);
                        },
                    };
                    current_text.push(c);
                    state = State::STRING { quote_type, escaping: false };
                    continue;
                }
                if c == '\\' {
                    state = State::STRING { quote_type, escaping: true };
                    continue;
                }
                if c == quote_type {
                    lexer_loc_warning!(loc, "current_text: {} ({}: {})", current_text, prev_loc.line_number, prev_loc.column_number);
                    prev_loc = loc.clone();
                    let tok = add_tok!();
                    state = State::NOP;
                    current_text = String::new();
                    match tok {
                        Some(tok) => {
                            result.push(tok);
                        },
                        None => {},
                    }
                    continue;
                }
                current_text.push(c);
                continue;
            },
            _ => {}
        }
        if false
            || c == '\n'
            || c == '\x0B'
            || c == '\x0C'
        {
            loc.line_number += 1;
            loc.column_number = config.starting_column_number - 1;
            if !matches!(state, State::NOP) && !current_text.is_empty() {
                eprintln!("{}: {}: ({}: {}): adding newline: \"{}\"", loc.line_number, loc.column_number, prev_loc.line_number, prev_loc.column_number, current_text);
                let tok = add_tok!();
                prev_loc.column_number += 1;
                current_text = String::new();
                match tok {
                    Some(tok) => {
                        result.push(tok);
                    }
                    None => {},
                }
                state = State::SPECCHAR;
            }
            prev_loc = loc.clone();
            continue;
        }
        if !current_text.is_empty() && matches!(state, State::NOP) {
            state = State::SPECCHAR;
            prev_loc = loc.clone();
            prev_loc.column_number += 1;
            continue;
        }
        loc.column_number += 1;
        if false
            || c.is_lowercase()
            || c.is_uppercase()
            || c.is_ascii_digit()
            || c == '-'
            || c == '_'
        {
            if !matches!(state, State::IDENTIFIER) {
                if !current_text.is_empty() {
                    eprintln!("{}: {}: ({}: {}): adding identifier: \"{}\"", loc.line_number, loc.column_number, prev_loc.line_number, prev_loc.column_number, current_text);
                    let tok = add_tok!();
                    current_text = String::new();
                    match tok {
                        Some(tok) => {
                            result.push(tok);
                        }
                        None => {},
                    }
                }
                prev_loc = loc.clone();
            }
            state = State::IDENTIFIER;
            current_text.push(c);
            continue;
        }
        if c.is_whitespace() {
            if !current_text.is_empty() {
                eprintln!("{}: {}: ({}: {}): adding whitespace: \"{}\"", loc.line_number, loc.column_number, prev_loc.line_number, prev_loc.column_number, current_text);
                let tok = add_tok!();
                current_text = String::new();
                match tok {
                    Some(tok) => {
                        result.push(tok);
                    },
                    None => {},
                }
            }
            prev_loc = loc.clone();
            state = State::WHITESPACE;
            continue;
        }
        if false
            || c == '\''
            || c == '"'
            || c == '`'
        {
            if !current_text.is_empty() {
                eprintln!("{}: {}: ({}: {}): adding string: \"{}\"", loc.line_number, loc.column_number, prev_loc.line_number, prev_loc.column_number, current_text);
                let tok = add_tok!();
                prev_loc = loc.clone();
                current_text = String::new();
                match tok {
                    Some(tok) => {
                        result.push(tok);
                    }
                    None => {},
                }
            }
            state = State::STRING {
                quote_type: c,
                escaping: false,
            };
            continue;
        }
        eprintln!("{}: {}: ({}: {}): adding specchar: \"{}\"", loc.line_number, loc.column_number, prev_loc.line_number, prev_loc.column_number, current_text);
        let mut tok = add_tok!();
        if current_text.is_empty() {
            tok = None;
        }
        current_text = String::new();
        match tok {
            Some(tok) => {
                result.push(tok);
            }
            None => {},
        }
        state = State::SPECCHAR;
        current_text.push(c);
        prev_loc = loc.clone();
    }
    return result;
}

#[derive(Debug)]
enum Item {
    Text(String),
    Item,
}

fn generate_hashmap(program_name: String, config: Config) -> ::std::collections::HashMap<String, Item> {
    #[allow(unused_macros)]
    macro_rules! generator_error {
        ($($msg: expr),+) => {
            error!("generator: {}", format!($($msg,)+));
        };
    }

    #[allow(unused_macros)]
    macro_rules! generator_loc_error {
        ($tok: expr, $($msg: expr),+) => {
            generator_error!("{}: {}: {}: {}", $tok.loc.filename, $tok.loc.line_number, $tok.loc.column_number, format!($($msg,)+));
        };
    }

    #[allow(unused_macros)]
    macro_rules! generator_warning {
        ($($msg: expr),+) => {
            warning!("generator: {}", format!($($msg,)+));
        };
    }

    #[allow(unused_macros)]
    macro_rules! generator_loc_warning {
        ($tok: expr, $($msg: expr),+) => {
            generator_warning!("{}: {}: {}: {}", $tok.loc.filename, $tok.loc.line_number, $tok.loc.column_number, format!($($msg,)+));
        };
    }

    #[derive(Debug, Clone)]
    enum State {
        #[allow(non_camel_case_types)] KEY,
        #[allow(non_camel_case_types)] COLON { key: String },
        #[allow(non_camel_case_types)] VALUE { key: String },
        #[allow(non_camel_case_types)] ONE_LINE_COMMENT { line_number: usize, previous_state: Box<State> },
    }
    let mut state = State::KEY;
    let input_file = match ::std::fs::read_to_string(config.input_file_name.clone()) {
        Ok(text) => text,
        Err(e) => {
            generator_error!("Cannot read file: {}", e);
            ::std::process::exit(2);
        }
    };
    let mut result = ::std::collections::HashMap::<String, Item>::new();
    let lexed = lex(program_name, config, input_file);
    eprintln!("{:#?}", lexed);
    for tok in lexed {
        eprintln!("{}: {}: {}: \"{:?}\"", tok.loc.filename, tok.loc.line_number, tok.loc.column_number, tok.value);
        match state.clone() {
            State::ONE_LINE_COMMENT { line_number, previous_state } => {
                if tok.loc.line_number != line_number {
                    state = *previous_state;
                } else {
                    continue;
                }
            },
            _ => {},
        }
        let specchar_good: bool;
        match tok.value {
            TokValue::SPECCHAR { ref value } => {
                match value.as_str() {
                    "#" => {
                        state = State::ONE_LINE_COMMENT {
                            line_number: tok.loc.line_number,
                            previous_state: Box::new(state),
                        };
                        specchar_good = false;
                    },
                    _ => {
                        specchar_good = true;

                    },
                }
            },
            _ => {
                specchar_good = true;
            }
        }
        if specchar_good {
            match state {
                State::KEY => {
                    match tok.value {
                        TokValue::IDENTIFIER {
                            value: identifier,
                        } => {
                            state = State::COLON { key: identifier };
                        }
                        _ => {
                            generator_loc_error!(tok, "Expected identifier");
                            ::std::process::exit(3);
                        }
                    }
                },
                State::COLON { key } => {
                    match tok.value {
                        TokValue::SPECCHAR {
                            value: specchar
                        } => {
                            if specchar == ":" {
                                state = State::VALUE { key };
                                continue;
                            }
                            generator_loc_error!(tok, "Expected `:`, found {}", specchar);
                            ::std::process::exit(3);
                        }
                        _ => {
                            generator_loc_error!(tok, "Expected specchar");
                            ::std::process::exit(3);
                        }
                    }
                },
                State::VALUE { ref key } => {
                    match tok.value {
                        TokValue::STRING {
                            value: text,
                            quote_type: _,
                        } => {
                            result.insert(key.to_string(), Item::Text(text));
                            state = State::KEY;
                        },
                        _ => {
                            generator_loc_error!(tok, "Expected string");
                            ::std::process::exit(3);
                        }
                    }
                },
                _ => unreachable!(),
            }
        }
    }
    return result;
}

fn generate_output(program_name: String, hashmap: ::std::collections::HashMap<String, Item>) -> String {
    eprintln!("hashmap: {:#?}", hashmap);
    todo!();
}

fn main() {
    let mut args = ::std::env::args();
    if args.len() == 0 {
        error!("Cannot read command-line arguments");
        eprintln!();
        usage!(PROGRAM_NAME);
        ::std::process::exit(1);
    }
    let program_name = match args.next() {
        Some(program_name) => {
            program_name
        }
        None => {
            error!("Cannot read first command-line argument");
            eprintln!();
            usage!(PROGRAM_NAME);
            ::std::process::exit(1);
        }
    };
    if args.len() == 0 {
        error!("No file provided");
        eprintln!();
        usage!(program_name);
        ::std::process::exit(1);
    }
    let config = parse_cla(program_name.clone(), args);
    let hashmap = generate_hashmap(program_name.clone(), config);
    let output = generate_output(program_name, hashmap);
    println!("{}", output);
}
