use std::{
    env::args,
    fs::File,
    io::{self, BufReader, Read, Write},
};

fn main() {
    if args().len() > 2 {
        println!("Usage: rlox [script]");
    } else if args().len() == 2 {
        run_file(args().nth(1).unwrap());
    } else {
        run_prompt();
    }
}

fn run_file(file_name: String) {
    let file = File::open(file_name).expect("open file");
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).expect("read file");
    run(&buffer);
}

fn run_prompt() {
    let mut buffer = String::new();

    loop {
        buffer.clear();
        print!("> ");
        io::stdout().flush().expect("flush");
        io::stdin().read_line(&mut buffer).expect("read line");
        if buffer.is_empty() {
            return;
        }
        run(&buffer);
    }
}

fn run(src: &str) {
    println!("{}", src.trim());
}
