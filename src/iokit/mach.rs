// mach/kern_return.h

#[allow(non_camel_case_types)]
pub type kern_return_t = std::os::raw::c_int;

pub const KERN_SUCCESS: kern_return_t = 0;

// mach/mach_init.h

#[allow(non_camel_case_types)]
pub type mach_port_t = u32;

extern "C" {
    pub static mach_task_self_: mach_port_t;
}
