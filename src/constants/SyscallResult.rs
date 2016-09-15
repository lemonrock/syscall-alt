// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


#[cfg(target_os = "bitrig")] pub use ::constants::bitrig::SyscallResult::*;
#[cfg(target_os = "dragonfly")] pub use ::constants::dragonfly::SyscallResult::*;
#[cfg(target_os = "freebsd")] pub use ::constants::freebsd::SyscallResult::*;
#[cfg(any(target_os = "android", target_os = "linux"))] pub use ::constants::linux_like::SyscallResult::*;
#[cfg(any(target_os = "ios", target_os = "macos"))] pub use ::constants::macosx_like::SyscallResult::*;
#[cfg(target_os = "netbsd")] pub use ::constants::netbsd::SyscallResult::*;
#[cfg(target_os = "openbsd")] pub use ::constants::openbsd::SyscallResult::*;
#[cfg(target_os = "solaris")] pub use ::constants::solaris::SyscallResult::*;
#[cfg(windows)] pub use ::constants::windows::SyscallResult::*;
