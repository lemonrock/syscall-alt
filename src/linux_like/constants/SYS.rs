// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "aarch64"))] pub use ::linux_like::constants::aarch64::SYS::*;
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "arm"))] pub use ::linux_like::constants::arm::SYS::*;
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "mips"))] pub use ::linux_like::constants::mips::SYS::*;
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "mips64"))] pub use ::linux_like::constants::mips64::SYS::*;
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "powerpc"))] pub use ::linux_like::constants::powerpc::SYS::*;
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "powerpc64"))] pub use ::linux_like::constants::powerpc64::SYS::*;
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "s390x"))] pub use ::linux_like::constants::s390x::SYS::*;
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "x86"))] pub use ::linux_like::constants::x86::SYS::*;
#[cfg(all(any(target_os = "linux", target_os = "android"), target_arch = "x86_64"))] pub use ::linux_like::constants::x86_64::SYS::*;
