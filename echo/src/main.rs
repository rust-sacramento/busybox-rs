use std::env::args;

fn main() {
    // TODO: Capture args
    let args: Vec<String> = args().collect();

    // TODO: Match "--version", "--help", "-n"
    match &*args[1] {
        "--version" => print_version(),
        "--help" => print_help(&args[0]),
        _ => handle_args(args),
    }

    // TODO: Print input as String
}

fn print_version() {
    println!("echo 0.1.0");
}

fn print_help(path: &str) {
    println!(
        "\n\
         Usage: {path} [SHORT-OPTION]... [STRING]...\n  \
           or: {path} LONG-OPTION\n\
         Echo the STRING(s) to standard output.\n\n  \
         -n     do not output the trainling newline\n\n  \
         --help     display this help and exit\n  \
         --version      output version information and exit\n
        ", 
        path=path);
}

fn handle_args(args: Vec<String>) {
    println!("{:?}", args);
}