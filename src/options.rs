use getopts::{Fail, Matches, Options};

pub fn get_options(args: Vec<String>) -> Result<Matches, Fail> {
    let mut opts = Options::new();

    opts.optopt("n", "name", "set container name", "CONTAINER_NAME");
    opts.optopt("", "exec", "exec command", "COMMAND");
    opts.optflag("h", "help", "print help message");
    opts.optflag("", "init", "exec pacstrap");
    opts.optflag("", "del", "delete container");
    opts.optflag("b", "create-bridge", "create brige");
    opts.optflag("b", "delete-bridge", "delete brige");

    opts.parse(&args[1..])
}
