// https://github.com/prometheus/node_exporter/blob/master/collector/cpu_darwin.go

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unsafe_code)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unknown_lints)]

use crate::error::{Error, Result};
use std::io::{Cursor, Read};
use std::mem;
use std::slice;

#[allow(clippy::all)]
mod C {
    include!(concat!(env!("OUT_DIR"), "/ffi/macos/cpu.rs"));
}

const ClocksPerSec: u64 = C::CLK_TCK as u64;

unsafe fn do_get_cpu() -> Result<Vec<Cpu>> {
    let mut count: C::natural_t = 0;
    let mut cpuload: *mut C::processor_cpu_load_info_data_t = std::ptr::null_mut();
    let mut ncpu: C::mach_msg_type_number_t = 0;

    let status = unsafe {
        C::host_processor_info(
            C::mach_host_self(),
            C::PROCESSOR_CPU_LOAD_INFO as i32,
            &mut ncpu,
            &mut cpuload as *mut _ as *mut _,
            &mut count,
        )
    };
    if status != C::KERN_SUCCESS as i32 {
        return Err(Error::FFIError(format!(
            "host_processor_info failed: {}",
            status
        )));
    }

    let cpu_ticks = unsafe { slice::from_raw_parts(cpuload as *const u32, ncpu as usize * 4) };

    let mut cpus = Vec::with_capacity(ncpu as usize);
    for chunk in cpu_ticks.chunks(4) {
        cpus.push(Cpu {
            clocks_per_sec: ClocksPerSec,
            user: chunk[C::CPU_STATE_USER as usize] as u64,
            system: chunk[C::CPU_STATE_SYSTEM as usize] as u64,
            nice: chunk[C::CPU_STATE_NICE as usize] as u64,
            idle: chunk[C::CPU_STATE_IDLE as usize] as u64,
        });
    }

    Ok(cpus)
}

pub fn get_cpu() -> Result<Vec<Cpu>> {
    let cpus = unsafe { do_get_cpu()? };
    Ok(cpus)
}

#[derive(Debug)]
pub struct Cpu {
    pub clocks_per_sec: u64,
    pub user: u64,
    pub system: u64,
    pub nice: u64,
    pub idle: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu() {
        let cpus = get_cpu().unwrap();
        cpus.iter().for_each(|cpu| {
            println!("{:?}", cpu);
        });
    }
}
