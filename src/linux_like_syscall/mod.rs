// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


pub mod SyscallResults;

include!("SyscallResult.rs");
include!("SyscallArgument.rs");
include!("SyscallNumber.rs");

// We used to use cfg_if! but with the include! bug it makes code harder to maintain
#[cfg(target_arch = "aarch64")] mod aarch64;
#[cfg(target_arch = "aarch64")] use self::aarch64::*;
#[cfg(target_arch = "arm")] mod arm;
#[cfg(target_arch = "arm")] use self::arm::*;
#[cfg(target_arch = "mips")] mod mips;
#[cfg(target_arch = "mips")] use self::mips::*;
#[cfg(target_arch = "mips")] mod mips64;
#[cfg(target_arch = "mips")] use self::mips64::*;
#[cfg(target_arch = "powerpc")] mod powerpc;
#[cfg(target_arch = "powerpc")] use self::powerpc::*;
#[cfg(target_arch = "powerpc64")] mod powerpc64;
#[cfg(target_arch = "powerpc64")] use self::powerpc64::*;
#[cfg(target_arch = "x86")] mod x86;
#[cfg(target_arch = "x86")] use self::x86::*;
#[cfg(target_arch = "x86_64")] mod x86_64;
#[cfg(target_arch = "x86_64")] use self::x86_64::*;
#[cfg(target_arch = "s390x")] mod s390x;
#[cfg(target_arch = "s390x")] use self::s390x::*;
//#[cfg(target_arch = "asmjs")] mod asmjs;
//#[cfg(target_arch = "asmjs")] use self::asmjs::*;
//#[cfg(target_arch = "wasm32")] mod wasm32;
//#[cfg(target_arch = "wasm32")] use self::wasm32::*;

include!("Syscall.rs");
