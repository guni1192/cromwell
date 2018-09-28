use getopts::{Fail, Matches, Options};

pub fn get_options(args: Vec<String>) -> Result<Matches, Fail> {
    let mut opts = Options::new();
    opts.optopt("", "path", "set container path", "CONTAINER PATH");
    opts.optopt("", "exec", "exec command", "EXEC");
    opts.optflag("h", "help", "print help message");
    opts.optflag("", "init", "exec pacstrap");

    opts.parse(&args[1..])
}
