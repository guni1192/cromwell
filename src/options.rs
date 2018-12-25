use getopts::{Fail, Matches, Options};

// TODO: clapに置き換え予定
// $ cromwell network <options>

pub fn get_network_options(args: Vec<String>) -> Result<Matches, Fail> {
    let mut opts = Options::new();

    opts.optopt("n", "name", "set container name", "CONTAINER_NAME");
    opts.optflag("", "clean", "cleanup network interface");
    opts.optflag("", "create-bridge", "create ace0 bridge");
    opts.optflag("", "delete-bridge", "delete ace0 bridge");

    opts.parse(&args[1..])
}
