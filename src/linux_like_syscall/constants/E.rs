// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


#[cfg(target_arch = "aarch64")] pub use ::linux_like_syscall::constants::aarch64::E::*;
#[cfg(target_arch = "arm")] pub use ::linux_like_syscall::constants::arm::E::*;
#[cfg(target_arch = "mips")] pub use ::linux_like_syscall::constants::mips::E::*;
#[cfg(target_arch = "mips64")] pub use ::linux_like_syscall::constants::mips64::E::*;
#[cfg(target_arch = "powerpc")] pub use ::linux_like_syscall::constants::powerpc::E::*;
#[cfg(target_arch = "powerpc64")] pub use ::linux_like_syscall::constants::powerpc64::E::*;
#[cfg(target_arch = "s390x")] pub use ::linux_like_syscall::constants::s390x::E::*;
#[cfg(target_arch = "x86")] pub use ::linux_like_syscall::constants::x86::E::*;
#[cfg(target_arch = "x86_64")] pub use ::linux_like_syscall::constants::x86_64::E::*;
