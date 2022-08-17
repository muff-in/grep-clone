use std::env;
use std::process;

use grep_clone::Config;

fn main() {

    let args: Vec<String> = env::args().collect(); 

    let config = Config::new(&args).unwrap_or_else(|err | {
        eprintln!("Error while parsing the arguement: {}", err);
        process::exit(1);
    });

    print!("Searching for {}", config.query);
    println!(" in file {}", config.filename);

    if let Err(e) = grep_clone::run(config)  {

        eprintln!("Application Error: {}", e);
        process::exit(1);
        
    }
    
}