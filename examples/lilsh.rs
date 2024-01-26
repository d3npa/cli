/// lilsh - little shell (just an example rly)
use cli::prompt;

fn print_help() {
    println!(
        "
            help - prints this help
            ping - prints 'pong!'
            echo - prints back (echoes) any arguments passed
            exit - cleans up and exits
        "
    );
}

fn main() {
    loop {
        prompt("> ", |command, args| match command {
            "help" => print_help(),
            "ping" => println!("pong!"),
            "echo" => println!("{}", &args[1..].join(" ")),
            "exit" => {
                println!("cleaning up...");
                // do cleany up things
                std::process::exit(0);
            }
            _ => println!("unknown command: '{command}'"),
        });
    }
}
