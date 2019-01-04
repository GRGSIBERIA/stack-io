extern crate getopts;
use std::{env, process};
use getopts::Options;

#[derive(Debug)]
struct Args {
    input: Vec<String>,
    output: Option<String>,
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("i", "import", "import file/directory from stack-io");
    opts.optflag("r", "read", "read file/directory from stack-io");
    opts.optflag("d", "directory", "import directory");
    opts.optflag("r", "recursive", "recursive to import directory");
    opts.optopt("f", "file", "target file", "NAME");
    opts.optopt("n", "number", "number of file id", "ID");
    opts.optopt("D", "dbpath", "database path", "DBPATH");
    opts.optflag("h", "help", "print this help menu");
    
    let matches = opts.parse(&args[1..]).unwrap_or_else(|f| panic!(f.to_string()));

    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }

    if matches.free.is_empty() {
        print_usage(&program, &opts);
    }

    Args {
        input: matches.free.clone(),
        output: matches.opt_str("f"),
    }
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);
}
