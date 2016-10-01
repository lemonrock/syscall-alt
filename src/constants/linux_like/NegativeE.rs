// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


#[cfg(target_arch = "aarch64")] pub use ::constants::linux_like::aarch64::NegativeE::*;
#[cfg(target_arch = "arm")] pub use ::constants::linux_like::arm::NegativeE::*;
#[cfg(target_arch = "mips")] pub use ::constants::linux_like::mips::NegativeE::*;
#[cfg(target_arch = "mips64")] pub use ::constants::linux_like::mips64::NegativeE::*;
#[cfg(target_arch = "powerpc")] pub use ::constants::linux_like::powerpc::NegativeE::*;
#[cfg(target_arch = "powerpc64")] pub use ::constants::linux_like::powerpc64::NegativeE::*;
#[cfg(target_arch = "s390x")] pub use ::constants::linux_like::s390x::NegativeE::*;
#[cfg(target_arch = "x86")] pub use ::constants::linux_like::x86::NegativeE::*;
#[cfg(target_arch = "x86_64")] pub use ::constants::linux_like::x86_64::NegativeE::*;
