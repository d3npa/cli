#[allow(unused_imports)]
use std::io::{stdin, stdout, Read, Write};

/// prompts user for a command, handled in `handler` as (command, args).
/// note: &args[0] is the command, like execve.
/// note: control-D will cause the command 'exit' to be run.
/// panics on IO errors
pub fn prompt<F>(prompt: &str, handler: F)
where
    F: Fn(&str, Vec<String>),
{
    stdout()
        .write_all(prompt.as_bytes())
        .expect("writing to stdout");
    stdout().flush().expect("flushing stdout");

    let mut line = String::new();

    if stdin().read_line(&mut line).expect("reading stdin") == 0 {
        // control-D
        handler("exit", vec!["exit".into()]);
        return;
    }

    let args: Vec<String> = line.split_whitespace().map(|s| s.to_owned()).collect();
    let command = &args.get(0).unwrap_or(&String::new()).clone();

    if command.is_empty() {
        return;
    }

    handler(&command, args);
}
