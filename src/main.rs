extern crate clap;
use clap::{Arg, App};

pub mod term;
pub mod syntax;
use term::*;

use std::io;
use std::io::prelude::*;
use std::fs::File;

// Because `?` doesn't format strings nicely
fn get_result<T>(name : Vec<u8>, result : Result<T, String>) -> T {
    match result {
        Ok(result) => result,
        Err(err) => {
            println!("[Error on `{}`]\n{}", String::from_utf8_lossy(&name), err);
            std::process::exit(0);
        }
    }
}

fn get_term(name : &Vec<u8>, defs : &Defs) -> Term {
    match defs.get(name) {
        Some(term) => term.clone(),
        None => {
            println!("[Error]\nTerm `{}` not found", String::from_utf8_lossy(&name));
            std::process::exit(0);
        }
    }
}

fn main() -> io::Result<()> {
    let matches = App::new("Formality")
        .version("0.1.0")
        .author("Victor Maia <srvictormaia@gmail.com>")
        .about("Formality")
        .arg(Arg::with_name("TYPE")
            .short("t")
            .long("type")
            .value_name("TYPE")
            .help("Infers the type of a term")
            .takes_value(true))
        .arg(Arg::with_name("EVAL")
            .short("e")
            .long("eval")
            .value_name("EVAL")
            .help("Evaluates a term")
            .takes_value(true))
        .arg(Arg::with_name("BOTH")
            .short("b")
            .long("both")
            .value_name("BOTH")
            .help("Evaluates and checks a term")
            .takes_value(true))
        .arg(Arg::with_name("FILE")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    // Reads the file to check / eval
    let file_name = matches.value_of("FILE").unwrap();
    let mut file = File::open(file_name)?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    // We are not using the main return value, so just add one
    code.extend_from_slice(b"\nType");

    let (_, defs) = get_result(b"main".to_vec(), syntax::term_from_ascii(code));

    // Infers the type of a term
    match matches.value_of("TYPE") {
        Some(term_name) => {
            let term_name = term_name.to_string().into_bytes();

            // Type-checks all dependencies
            for (nam, def) in &defs {
                get_result(nam.to_vec(), syntax::infer_with_string_error(&def, &defs, false, true));
            }

            // Loads the term
            let term = get_term(&term_name, &defs);

            // Prints its inferred type
            let t_ty = get_result(term_name, syntax::infer_with_string_error(&term, &defs, false, true));
            println!("{}", syntax::term_to_string(&t_ty, &mut Vec::new(), true));
        },
        None => {}
    }

    // Evals a term to normal form
    match matches.value_of("EVAL") {
        Some(term_name) => {
            let term_name = term_name.to_string().into_bytes();

            // Loads the term
            let term = get_term(&term_name, &defs);

            // Prints its normal form
            let mut t_nf = term.clone();
            reduce(&mut t_nf, &defs, true);
            println!("{}", syntax::term_to_string(&t_nf, &mut Vec::new(), true));
        },
        None => {}
    }

    // Evals and prints
    match matches.value_of("BOTH") {
        Some(term_name) => {
            let term_name = term_name.to_string().into_bytes();

            // Type-checks all dependencies
            for (nam, def) in &defs {
                get_result(nam.to_vec(), syntax::infer_with_string_error(&def, &defs, false, true));
            }

            // Loads the term
            let term = get_term(&term_name, &defs);

            // Prints it
            println!("[TERM]\n\n{}\n", syntax::term_to_string(&term, &mut Vec::new(), true));

            // Prints its inferred type
            let t_ty = get_result(term_name, syntax::infer_with_string_error(&term, &defs, false, true));
            println!("[TYPE]\n\n{}\n", syntax::term_to_string(&t_ty, &mut Vec::new(), true));

            // Prints its normal form
            let mut t_nf = term.clone();
            reduce(&mut t_nf, &defs, true);
            println!("[EVAL]\n\n{}\n", syntax::term_to_string(&t_nf, &mut Vec::new(), true));
        },
        None => {}
    }

    Ok(())
}
