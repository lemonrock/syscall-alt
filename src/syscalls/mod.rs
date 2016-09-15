// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


#[cfg(any(target_os = "bitrig", target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd"))] mod bsd_like;
#[cfg(any(target_os = "bitrig", target_os = "dragonfly", target_os = "freebsd", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "openbsd"))] pub use self::bsd_like::*;
#[cfg(any(target_os = "android", target_os = "linux"))] mod linux_like;
#[cfg(any(target_os = "android", target_os = "linux"))] pub use self::linux_like::*;
#[cfg(target_os = "solaris")] mod solaris;
#[cfg(target_os = "solaris")] pub use self::solaris::*;
#[cfg(windows)] mod windows;
#[cfg(windows)] pub use self::windows::*;
