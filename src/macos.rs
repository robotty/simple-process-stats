use crate::{Error, ProcessStats};
use std::time::Duration;

pub fn get_info() -> Result<ProcessStats, Error> {
    let pid = unsafe { libc::getpid() };
    let proc_info = darwin_libproc::task_info(pid).map_err(Error::SystemCall)?;
    let timebase_info = mach_timebase_info::get();

    Ok(ProcessStats {
        memory_usage_bytes: proc_info.pti_resident_size,
        cpu_time_user: timebase_info.mach_time_to_duration(proc_info.pti_total_user),
        cpu_time_kernel: timebase_info.mach_time_to_duration(proc_info.pti_total_system),
    })
}

#[repr(C)]
#[derive(Copy, Clone)]
struct mach_timebase_info {
    pub numer: u32,
    pub denom: u32,
}

impl mach_timebase_info {
    pub fn get() -> Self {
        extern "C" {
            fn mach_timebase_info(info: *mut mach_timebase_info) -> libc::c_int;
        }
        let mut info = mach_timebase_info { numer: 0, denom: 0 };
        unsafe {
            mach_timebase_info(&mut info);
        }
        info
    }

    pub fn mach_time_to_duration(self, time: u64) -> Duration {
        Duration::from_nanos(time * self.numer as u64 / self.denom as u64)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::macos;

    #[test]
    pub fn test_no_error() {
        #[no_mangle]
        fn spin_for_a_bit() {
            let mut _a = 0;
            for _i in 0..9999999 {
                _a += 1;
            }
        }

        // to get some nonzero number for cpu_time_user
        spin_for_a_bit();

        dbg!(macos::get_info().unwrap());
    }

    #[test]
    pub fn test_valid_timebase_info() {
        let timebase_info = super::mach_timebase_info::get();
        // this should be guaranteed by the kernel
        assert_ne!(timebase_info.numer, 0);
        assert_ne!(timebase_info.denom, 0);
    }
}
