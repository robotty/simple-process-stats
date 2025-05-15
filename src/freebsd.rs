use crate::{Error, ProcessStats};
use std::time::Duration;
use sysinfo::{get_current_pid, Pid, System};

pub fn get_info() -> Result<ProcessStats, Error> {
    let pid = match get_current_pid() {
        Ok(pid) => { pid }
        Err(e) => { panic!("failed to get current pid: {}", e) }
    };

    let s = System::new_all();
    if let Some(p) = s.process(Pid::from(pid)) {
        let memory_usage_bytes = p.memory();
        let user_mode_seconds = p.run_time();
        let kernel_mode_seconds = p.run_time();

        Ok(ProcessStats {
            cpu_time_user: Duration::from_secs_f64(user_mode_seconds as f64),
            cpu_time_kernel: Duration::from_secs_f64(kernel_mode_seconds as f64),
            memory_usage_bytes,
        })
    } else {
        Ok(
            ProcessStats {
                cpu_time_user: Duration::from_secs_f64(0 as f64),
                cpu_time_kernel: Duration::from_secs_f64(0 as f64),
                memory_usage_bytes: 0,
            }
        )
    }
}

#[cfg(test)]
pub mod tests {
    use crate::freebsd;

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

        dbg!(freebsd::get_info().unwrap());
    }
}
