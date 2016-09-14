// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


#[cfg(target_arch = "aarch64")] mod aarch64;
#[cfg(target_arch = "aarch64")] pub use self::aarch64::*;
#[cfg(target_arch = "arm")] mod arm;
#[cfg(target_arch = "arm")] pub use self::arm::*;
#[cfg(target_arch = "mips")] mod mips;
#[cfg(target_arch = "mips")] pub use self::mips::*;
#[cfg(target_arch = "mips64")] mod mips64;
#[cfg(target_arch = "mips64")] pub use self::mips64::*;
#[cfg(target_arch = "powerpc")] mod powerpc;
#[cfg(target_arch = "powerpc")] pub use self::powerpc::*;
#[cfg(target_arch = "powerpc64")] mod powerpc64;
#[cfg(target_arch = "powerpc64")] pub use self::powerpc64::*;
#[cfg(target_arch = "s390x")] mod s390x;
#[cfg(target_arch = "s390x")] pub use self::s390x::*;
#[cfg(target_arch = "x86")] mod x86;
#[cfg(target_arch = "x86")] pub use self::x86::*;
#[cfg(target_arch = "x86_64")] mod x86_64;
#[cfg(target_arch = "x86_64")] pub use self::x86_64::*;
