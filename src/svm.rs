use crate::println;

/// // Before enabling SVM, software should detect whether SVM can be enabled using the following algorithm:
/// if (CPUID Fn8000_0001_ECX[SVM] == 0)
///     return SVM_NOT_AVAIL;
///
/// if (VM_CR.SVMDIS == 0)
///     return SVM_ALLOWED;
///
/// if (CPUID Fn8000_000A_EDX[SVML]==0)
///     return SVM_DISABLED_AT_BIOS_NOT_UNLOCKABLE
///     // the user must change a platform firmware setting to enable SVM
/// else return SVM_DISABLED_WITH_KEY;
///     // SVMLock may be unlockable; consult platform firmware or TPM to obtain the key.
pub fn svm_support() -> bool {
    let cpuid = raw_cpuid::CpuId::new();
    // if (CPUID Fn8000_0001_ECX[SVM] == 0)
    //  return SVM_NOT_AVAIL;
    if let Some(_svmf) = cpuid.get_svm_info() {
        // println!("{:?}", svmf);
        return true;
    }
    false
}

pub fn enable_svm() {
    use x86_64::registers::model_specific::Efer;
    use x86_64::registers::model_specific::EferFlags; 

    unsafe { Efer::update(|flags| *flags |= EferFlags::SECURE_VIRTUAL_MACHINE_ENABLE) };
}

use x86_64::{
    structures::paging::{
        FrameAllocator as X86FrameAllocator, Size4KiB,
    },
};

pub fn vminit(frame_allocator: &mut impl X86FrameAllocator<Size4KiB>) {
    // Setup VMCB, save area, etc.
    let vmcb_frame = frame_allocator.allocate_frame().expect("No frames available for VMCB");
    let vmcb_phys_addr = vmcb_frame.start_address();
    println!("Allocated VMCB frame at physical address: {:#x}", vmcb_phys_addr.as_u64());

    // VMCB initialization

}

pub fn vmrun() {
    // use core::arch::asm;

    // unsafe {
    //     asm!(
    //         "vmrun rax, rbx",
    //         in("rax") guest_physical_address,
    //         in("rbx") vmcb_physical_address,
    //     );
    // }
}