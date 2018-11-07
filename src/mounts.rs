use nix::mount::{mount, MsFlags};
use nix::Error;

pub fn mount_proc() -> Result<(), Error> {
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::MS_MGC_VAL,
        None::<&str>,
    )
}

pub fn mount_rootfs() -> Result<(), Error> {
    mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_PRIVATE,
        None::<&str>,
    )
}

pub fn mount_container_path(container_path: &str) -> Result<(), Error> {
    mount(
        Some(container_path),
        container_path,
        None::<&str>,
        MsFlags::MS_BIND | MsFlags::MS_REC,
        None::<&str>,
    )
}
