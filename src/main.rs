extern crate getopts;
use std::{env, process};
use getopts::Options;

#[derive(Debug)]
struct Args {
    input: Vec<String>,
    import_file: Option<String>,
    read_id: Option<String>,
    dbpath: Option<String>,
    directory: bool,
    recursive: bool,
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
    opts.optopt("I", "import", "import file/directory from stack-io", "IMPORT_FILE");
    opts.optopt("R", "read", "read file/directory from stack-io", "READ_FILE");
    opts.optflag("d", "directory", "import directory");
    opts.optflag("r", "recursive", "recursive to import directory");
    opts.optopt("D", "dbpath", "database path", "DBPATH");
    opts.optflag("h", "help", "print this help menu");
    
    let matches = opts.parse(&args[1..]).unwrap_or_else(|f| panic!(f.to_string()));

    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }

    Args {
        input: matches.free.clone(),
        import_file: matches.opt_str("I"),
        read_id: matches.opt_str("R"),
        dbpath: matches.opt_str("D"),
        directory: matches.opt_present("d"),
        recursive: matches.opt_present("r"),
    }
}

fn main() {
    let args = parse_args();
    println!("{:?}", args);
}
