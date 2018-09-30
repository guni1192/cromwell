use getopts::{Fail, Matches, Options};

pub fn get_options(args: Vec<String>) -> Result<Matches, Fail> {
    let mut opts = Options::new();
    opts.optopt("", "name", "set container name", "CONTAINER_NAME");
    opts.optopt("", "exec", "exec command", "COMMAND");
    opts.optflag("h", "help", "print help message");
    opts.optflag("", "init", "exec pacstrap");

    opts.parse(&args[1..])
}
