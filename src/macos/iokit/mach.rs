// mach/mach_init.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/mach/mach_init.h

#[allow(non_camel_case_types)]
pub type mach_port_t = u32;

extern "C" {
    pub static mach_task_self_: mach_port_t;
}

// mach/mach_types.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/mach/mach_types.h

#[allow(non_camel_case_types)]
pub type task_port_t = mach_port_t;

// mach/i386/vm_types.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/mach/i386/vm_types.h

#[allow(non_camel_case_types)]
pub type mach_vm_address_t = u64;
#[allow(non_camel_case_types)]
pub type mach_vm_size_t = u64;
