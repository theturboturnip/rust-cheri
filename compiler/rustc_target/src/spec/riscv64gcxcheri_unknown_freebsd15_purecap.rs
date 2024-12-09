use crate::spec::{Cc, CodeModel, LinkerFlavor, Lld, PanicStrategy};
use crate::spec::{RelocModel, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::freebsd_base::opts();
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-march=rv64imafdcxcheri", "-mabi=l64pc128d", "-mno-relax"]);

    Target {
        // Amended from the RISC-V one and the morello one
        // data_layout_morello: "e-m:e-pf200:128:128:128:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-A200-P200-G200".into(),
        // data_layout_riscv:   "e-m:e-p:64:64-i64:64-i128:128-n64-S128".into(),
        // See https://llvm.org/docs/LangRef.html#data-layout to understand what's happening here
        // TODO the morello one used pf200:128:128:128:64. This specifies properties for pointers to addrspace 200.
        // Cheri-LLVM uses address space 200 for capability-addressable things, but I'm not sure what the "f" is doing here.
        // 
        // e
        //      little-endian
        // m:e
        //      elf mangling
        // pf200:128:128:128:64
        //      pointers for addrspace 200 are 128-bit size, 128-bit aligned, 128-bit preferred align, and provide 64-bit addressing
        // i64:64
        //      64-bit ints are 64-bit aligned
        // i128:128
        //      128-bit ints are 128-bit aligned
        // n64
        //      the native integer size is 64-bit
        // S128
        //      the stack is 128-byte aligned
        // A200
        //      alloca-allocated objects are in addrspace 200
        // P200
        //      program memory is in addrspace 200
        // G200
        //      global variables are in addrspace 200
        data_layout: "e-m:e-pf200:128:128:128:64-i64:64-i128:128-n64-S128-A200-P200-G200".into(),
        llvm_target: "riscv64-unknown-freebsd15".into(),
        pointer_width: 64,
        arch: "riscv64".into(),

        options: TargetOptions {
            pointer_type_width: Some(128),
            linker: Some("lld".into()),
            llvm_abiname: "l64pc128d".into(),
            // riscv64 plus imafd features, plus cheri, no relaxing (I am guessing using minus instead of plus means disable the feature?)
            features: "+m,+a,+f,+d,+c,+xcheri,-relax".into(),
            ..base
        },
    }
}
