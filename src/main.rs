use shy::App;
use std::{io, os::unix::process::CommandExt, panic, process::Command};

fn main() -> Result<(), io::Error> {
    if let Some(hostname) = run()? {
        std::env::set_var("TERM", "xterm"); // TODO xterm-kitty hack
        let mut cmd = Command::new("ssh");
        let cmd = cmd.arg(hostname);
        let err = cmd.exec();
        eprintln!("{:?}", err);
    }

    Ok(())
}

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
