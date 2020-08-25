use crate::{Error, ProcessStats};
use std::time::Duration;

pub fn get_info() -> Result<ProcessStats, Error> {
    let pid = unsafe { libc::getpid() };
    let proc_info = darwin_libproc::task_info(pid).map_err(Error::SystemCall)?;

    Ok(ProcessStats {
        memory_usage_bytes: proc_info.pti_resident_size,
        cpu_time_user: Duration::from_nanos(proc_info.pti_total_user),
        cpu_time_kernel: Duration::from_nanos(proc_info.pti_total_system)
    })
}