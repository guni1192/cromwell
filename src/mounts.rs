use nix::mount::{mount, MsFlags};
use nix::Error;

#[allow(dead_code)]
pub fn mount_proc() -> Result<(), Error> {
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::MS_MGC_VAL,
        None::<&str>,
    )
}
