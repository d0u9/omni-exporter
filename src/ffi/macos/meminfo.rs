#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unsafe_code)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unknown_lints)]

extern crate libc;

use crate::error::{Error, Result};
use std::mem;

#[allow(clippy::all)]
mod C {
    include!(concat!(env!("OUT_DIR"), "/ffi/macos/meminfo.rs"));
}

unsafe fn get_vmstat() -> Result<(u64, C::vm_statistics64_data_t)> {
    let host = unsafe { C::mach_host_self() };
    let mut vmstat: C::vm_statistics64_data_t = unsafe { mem::zeroed() };
    let mut info_count = libc::HOST_VM_INFO64_COUNT;
    let ret = unsafe {
        C::host_statistics64(
            host,
            C::HOST_VM_INFO64 as i32,
            &mut vmstat as *mut _ as *mut _,
            &mut info_count as *mut _,
        )
    };
    if ret != C::KERN_SUCCESS as i32 {
        return Err(Error::FFIError(format!(
            "host_statistics64 failed: {}",
            ret
        )));
    }

    let mut page_size: C::vm_size_t = 0;
    let ret = unsafe { C::host_page_size(host, &mut page_size) };
    if ret != C::KERN_SUCCESS as i32 {
        return Err(Error::FFIError(format!("host_page_size failed: {}", ret)));
    }

    Ok((page_size as u64, vmstat))
}

pub fn get_meminfo() -> Result<MemInfo> {
    let (page_size, vmstat) = unsafe { get_vmstat()? };

    println!("page_size: {}", page_size);
    println!("vmstat: {:?}", vmstat);

    Ok(MemInfo {
        page_size,
        active_bytes: vmstat.active_count as u64 * page_size,
        compressed_bytes: vmstat.compressor_page_count as u64 * page_size,
        inactive_bytes: vmstat.inactive_count as u64 * page_size,
        wired_bytes: vmstat.wire_count as u64 * page_size,
        free_bytes: vmstat.free_count as u64 * page_size,
        swapped_in_bytes_total: vmstat.pageins * page_size,
        swapped_out_bytes_total: vmstat.pageouts * page_size,
        internal_bytes: vmstat.internal_page_count as u64 * page_size,
        purgeable_bytes: vmstat.purgeable_count as u64 * page_size,
        total_bytes: 0,
        swap_used_bytes: 0,
        swap_total_bytes: 0,
    })
}

#[derive(Debug)]
pub struct MemInfo {
    pub page_size: u64,
    pub active_bytes: u64,
    pub compressed_bytes: u64,
    pub inactive_bytes: u64,
    pub wired_bytes: u64,
    pub free_bytes: u64,
    pub swapped_in_bytes_total: u64,
    pub swapped_out_bytes_total: u64,
    pub internal_bytes: u64,
    pub purgeable_bytes: u64,
    pub total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_meminfo() {
        get_meminfo().unwrap();
    }
}
