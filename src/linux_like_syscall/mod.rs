// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::linux_like_syscall::constants::SYS::*;


pub mod constants;
pub mod syscalls;
pub mod SyscallResults;


include!("SyscallResult.rs");
include!("SyscallArgument.rs");
include!("SyscallNumber.rs");
include!("Syscall.rs");
