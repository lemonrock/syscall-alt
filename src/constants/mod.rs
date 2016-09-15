// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


pub mod bitrig;

pub mod dragonfly;

pub mod freebsd;

pub mod linux_like;

pub mod macosx_like;

pub mod netbsd;

pub mod openbsd;

pub mod solaris;

pub mod windows;

/// For the current OS and architecture
pub mod E;

/// For the current OS and architecture
pub mod SYS;

/// For the current OS and architecture
pub mod SyscallResult;
