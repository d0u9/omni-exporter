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
use std::mem;

#[allow(clippy::all)]
mod C {
    include!(concat!(env!("OUT_DIR"), "/ffi/macos/cpu.rs"));
}

unsafe fn do_get_cpu() -> Result<Cpu> {
    let host = unsafe { C::mach_host_self() };

    let mut count: C::natural_t = 0;
    let mut cpuload: C::processor_cpu_load_info_data_t = unsafe { mem::zeroed() };
    let mut ncpu: C::mach_msg_type_number_t = 0;

    let status = unsafe { C::host_processor_info(
        host,
        libc::PROCESSOR_CPU_LOAD_INFO,
        &mut ncpu,
        &mut cpuload as *mut _ as *mut _,
        &mut count,
    )};

    if status != C::KERN_SUCCESS as i32 {
        return Err(Error::FFIError(format!("host_processor_info failed: {}", status)));
    }



    let target: C::vm_map_t = C::mach_task_self_;
    let address: C::vm_address_t = &cpuload as *const _ as C::vm_address_t;

    type CpuTicks = [u32; C::CPU_STATE_MAX as usize];
    let mut cpu_ticks: CpuTicks = unsafe { mem::zeroed() };
    let size = ncpu as usize * mem::size_of::<CpuTicks>();

    unsafe { C::vm_deallocate(target, address, size) };

    Ok(Cpu {
        user: 0,
        system: 0,
        nice: 0,
        idle: 0,
    })
}

pub fn get_cpu() -> Result<Cpu> {
    let cpu = unsafe { do_get_cpu()? };
    Ok(cpu)
}

#[derive(Debug)]
pub struct Cpu {
    user: u64,
    system: u64,
    nice: u64,
    idle: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu () {
        println!("{:?}", get_cpu().unwrap());
    }
}
