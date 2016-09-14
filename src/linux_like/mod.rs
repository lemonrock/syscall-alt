// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::linux_like::constants::SYS::*;
use ::SyscallArgument;
use ::SyscallNumber;
use ::SyscallResult;


pub mod constants;

#[cfg(all(any(target_os = "linux", target_os = "android"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64")))]
pub mod syscalls;

#[cfg(all(any(target_os = "linux", target_os = "android"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64")))]
pub mod SyscallResults;

#[cfg(all(any(target_os = "linux", target_os = "android"), any(target_arch = "aarch64", target_arch = "arm", target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "s390x", target_arch = "x86", target_arch = "x86_64")))]
include!("Syscall.rs");
