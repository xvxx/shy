use shy::App;
use std::{io, os::unix::process::CommandExt, panic, process::Command};

fn main() -> Result<(), io::Error> {
    let args = parse_args()?;

    if args.is_empty() {
        return print_usage();
    }

    for arg in args {
        match arg.as_ref() {
            "-h" | "-help" | "--help" => return print_usage(),
            "-v" | "-version" | "--version" => return print_version(),
            "-c" | "-config" | "--config" => todo!(),
            _ => {}
        }
    }

    if let Some(hostname) = run()? {
        std::env::set_var("TERM", "xterm"); // TODO xterm-kitty hack
        let mut cmd = Command::new("ssh");
        let cmd = cmd.arg(hostname);
        let err = cmd.exec();
        eprintln!("{:?}", err);
    }

    Ok(())
}

/// Run the app, optionally returning a host to SSH to.
fn run() -> Result<Option<String>, io::Error> {
    setup_panic_hook();
    let mut app = App::new()?;
    Ok(app.run()?)
}

/// We need to cleanup the terminal before exiting, even on panic!
fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        println!("{}", panic_info);
    }));
}

/// Converts -c=file into ["-c", "file"]
fn parse_args() -> Result<Vec<String>, io::Error> {
    let mut args = vec![];
    for arg in std::env::args().skip(1).collect::<Vec<String>>() {
        if arg.starts_with('-') && arg.contains('=') {
            for part in arg.split("=") {
                args.push(part.to_string());
            }
        } else {
            args.push(arg);
        }
    }
    Ok(args)
}

/// --help
fn print_usage() -> Result<(), io::Error> {
    println!(
        "usage: shy [options]

Options:
    -c, --config FILE    Use config file of ~/.ssh/config
    -v, --version        Print shy version and exit.
    -h, --help           Show this message."
    );
    Ok(())
}

/// --version
fn print_version() -> Result<(), io::Error> {
    println!("shy v{}", shy::VERSION);
    Ok(())
}
