/// Read the RIP register (instruction pointer).
#[inline(always)]
pub fn rip() -> u64 {
    let rip: u64;
    unsafe {
        llvm_asm!("leaq 0(%rip), $0" : "=r" (rip) ::);
    }
    rip
}

/// Read the RSP register (stack pointer register).
#[inline(always)]
pub fn rsp() -> u64 {
    let rsp: u64;
    unsafe {
        llvm_asm!("mov %rsp, $0" : "=r" (rsp) ::);
    }
    return rsp;
}

/// Read the RBP register (base pointer register).
#[inline(always)]
pub fn rbp() -> u64 {
    let rbp: u64;
    unsafe {
        llvm_asm!("mov %rbp, $0" : "=r" (rbp) ::);
    }
    return rbp;
}
