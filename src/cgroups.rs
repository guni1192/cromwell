use cgroups::cgroup_builder::CgroupBuilder;
use cgroups::cpu::CpuController;
use cgroups::{hierarchies, Cgroup, CgroupPid, Controller};

use nix::pty::SessionId;
use nix::unistd::Pid;

pub fn apply_cgroups(pid: Pid) {
    let v1 = hierarchies::V1::new();
    let cgroup: Cgroup = CgroupBuilder::new("vagrant", &v1).cpu().done().build();

    let cpus: &CpuController = cgroup
        .controller_of()
        .expect("Could not get cpu controller");

    cpus.set_cfs_period(100000)
        .expect("Could not set_cfs_period");
    cpus.set_cfs_quota(30000).expect("Could not set_cfs_quota");

    let pid = CgroupPid::from(SessionId::from(pid) as u64);
    cpus.add_task(&pid).unwrap();
}
