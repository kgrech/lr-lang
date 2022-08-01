#[macro_use]
extern crate lalrpop_util;

mod ast;
mod runtime;

use crate::runtime::executor::execute_program;
use crate::runtime::frame::Frame;
use clap::Parser;
use std::fs;

#[derive(Debug, Parser)]
#[clap(name = "lr language interpreter", about, verbatim_doc_comment)]
struct Args {
    #[clap(short, long)]
    program_file: String,
}

fn main() {
    let args: Args = Args::parse();
    println!("Running program {}", args.program_file);

    let program_text =
        fs::read_to_string(args.program_file).expect("Unable to read the program file");
    let program = ast::lr_lang::ProgramParser::new()
        .parse(&program_text)
        .expect("Unable to parse the program file");
    let mut root_frame = Frame::default();
    execute_program(&mut root_frame, &program).unwrap();
    println!("Main frame: {:#?}", root_frame);
}
