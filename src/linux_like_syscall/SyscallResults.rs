// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


extern crate libc;
use ::linux_like_syscall::SyscallResult;


pub const EACCES: SyscallResult = -(self::libc::EACCES as SyscallResult);
pub const EAGAIN: SyscallResult = -(self::libc::EAGAIN as SyscallResult);
pub const EBADF: SyscallResult = -(self::libc::EBADF as SyscallResult);
pub const ECONNABORTED: SyscallResult = -(self::libc::ECONNABORTED as SyscallResult);
pub const ECONNRESET: SyscallResult = -(self::libc::ECONNRESET as SyscallResult);
pub const EDESTADDRREQ: SyscallResult = -(self::libc::EDESTADDRREQ as SyscallResult);
pub const EFAULT: SyscallResult = -(self::libc::EFAULT as SyscallResult);
pub const EINTR: SyscallResult = -(self::libc::EINTR as SyscallResult);
pub const EINVAL: SyscallResult = -(self::libc::EINVAL as SyscallResult);
pub const EIO: SyscallResult = -(self::libc::EIO as SyscallResult);
pub const EMFILE: SyscallResult = -(self::libc::EMFILE as SyscallResult);
pub const EMSGSIZE: SyscallResult = -(self::libc::EMSGSIZE as SyscallResult);
pub const ENETDOWN: SyscallResult = -(self::libc::ENETDOWN as SyscallResult);
pub const ENETUNREACH: SyscallResult = -(self::libc::ENETUNREACH as SyscallResult);
pub const ENFILE: SyscallResult = -(self::libc::ENFILE as SyscallResult);
pub const ENOBUFS: SyscallResult = -(self::libc::ENOBUFS as SyscallResult);
pub const ENOMEM: SyscallResult = -(self::libc::ENOMEM as SyscallResult);
pub const ENOSPC: SyscallResult = -(self::libc::ENOSPC as SyscallResult);
pub const ENOSR: SyscallResult = -(self::libc::ENOSR as SyscallResult);
pub const ENOTCONN: SyscallResult = -(self::libc::ENOTCONN as SyscallResult);
pub const ENOTSOCK: SyscallResult = -(self::libc::ENOTSOCK as SyscallResult);
pub const EOPNOTSUPP: SyscallResult = -(self::libc::EOPNOTSUPP as SyscallResult);
pub const EPERM: SyscallResult = -(self::libc::EPERM as SyscallResult);
pub const EPIPE: SyscallResult = -(self::libc::EPIPE as SyscallResult);
pub const EPROTO: SyscallResult = -(self::libc::EPROTO as SyscallResult);
pub const EPROTONOSUPPORT: SyscallResult = -(self::libc::EPROTONOSUPPORT as SyscallResult);
pub const ESOCKTNOSUPPORT: SyscallResult = -(self::libc::ESOCKTNOSUPPORT as SyscallResult);
pub const ETIMEDOUT: SyscallResult = -(self::libc::ETIMEDOUT as SyscallResult);
