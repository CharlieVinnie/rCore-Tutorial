//! Process management syscalls
use crate::{task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, current_user_token, current_trap_cx, current_memory_set}, timer::get_time_us, mm::{translated_byte_buffer, VirtAddr, PageTable, PhysAddr, MapPermission}};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let time_us = get_time_us();
    let time = TimeVal {
        sec: time_us / 1_000_000,
        usec: time_us % 1_000_000,
    };
    let current_token = current_user_token();
    let buffer = translated_byte_buffer(current_token, ts as *const u8, core::mem::size_of::<TimeVal>());
    let src = unsafe {
        core::slice::from_raw_parts(&time as *const TimeVal as *const u8, core::mem::size_of::<TimeVal>())
    };
    let mut offset = 0;
    for dst in buffer {
        let len = dst.len();
        dst.copy_from_slice(&src[offset..offset+len]);
        offset += len;
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 | 1 => {
            let va = VirtAddr::from(id);
            let vpn = va.floor();
            let Some(pte) = PageTable::from_token(current_user_token()).translate(vpn) else {
                return -1;
            };
            let pa = PhysAddr::from(pte.ppn()).0 + va.page_offset();
            match trace_request {
                0 => {
                    if !pte.readable() || !pte.user_visible() {
                        -1
                    } else {
                        unsafe { (pa as *const u8).read_volatile() as isize }
                    }
                },
                1 => {
                    if !pte.readable() || !pte.writable() || !pte.user_visible() {
                        -1
                    } else {
                        unsafe { (pa as *mut u8).write_volatile(data as u8); }
                        0
                    }
                },
                _ => unreachable!(),
            }
        },
        2 => {
            current_trap_cx().syscall_trace[id] as isize
        },
        _ => {
            -1
        }
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    trace!("kernel: sys_mmap");
    let va_start = VirtAddr::from(start);
    if !va_start.aligned() {
        return -1;
    }
    let va_end = VirtAddr::from(start + len);
    if (prot & !0x7) != 0 || (prot & 0x7) == 0 {
        return -1;
    }
    let map_permission = MapPermission::from_bits((prot as u8) << 1).unwrap() | MapPermission::U;
    let mut memory_set = current_memory_set();
    if memory_set.insert_framed_area(va_start, va_end, map_permission).is_ok() {
        0
    } else {
        -1
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    let va_start = VirtAddr::from(start);
    let va_end = VirtAddr::from(start + len);
    if !va_start.aligned() {
        return -1;
    }
    let mut memory_set = current_memory_set();
    if memory_set.pop(va_start, va_end).is_ok() {
        0
    } else {
        -1
    }
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
