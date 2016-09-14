// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::PosixErrorNumber;

pub const EPERM: PosixErrorNumber = 1;
pub const ENOENT: PosixErrorNumber = 2;
pub const ESRCH: PosixErrorNumber = 3;
pub const EINTR: PosixErrorNumber = 4;
pub const EIO: PosixErrorNumber = 5;
pub const ENXIO: PosixErrorNumber = 6;
pub const E2BIG: PosixErrorNumber = 7;
pub const ENOEXEC: PosixErrorNumber = 8;
pub const EBADF: PosixErrorNumber = 9;
pub const ECHILD: PosixErrorNumber = 10;
pub const EAGAIN: PosixErrorNumber = 11;
pub const ENOMEM: PosixErrorNumber = 12;
pub const EACCES: PosixErrorNumber = 13;
pub const EFAULT: PosixErrorNumber = 14;
pub const EBUSY: PosixErrorNumber = 16;
pub const EEXIST: PosixErrorNumber = 17;
pub const EXDEV: PosixErrorNumber = 18;
pub const ENODEV: PosixErrorNumber = 19;
pub const ENOTDIR: PosixErrorNumber = 20;
pub const EISDIR: PosixErrorNumber = 21;
pub const EINVAL: PosixErrorNumber = 22;
pub const ENFILE: PosixErrorNumber = 23;
pub const EMFILE: PosixErrorNumber = 24;
pub const ENOTTY: PosixErrorNumber = 25;
pub const EFBIG: PosixErrorNumber = 27;
pub const ENOSPC: PosixErrorNumber = 28;
pub const ESPIPE: PosixErrorNumber = 29;
pub const EROFS: PosixErrorNumber = 30;
pub const EMLINK: PosixErrorNumber = 31;
pub const EPIPE: PosixErrorNumber = 32;
pub const EDOM: PosixErrorNumber = 33;
pub const ERANGE: PosixErrorNumber = 34;
pub const EDEADLK: PosixErrorNumber = 36;
pub const EDEADLOCK: PosixErrorNumber = 36;
pub const ENAMETOOLONG: PosixErrorNumber = 38;
pub const ENOLCK: PosixErrorNumber = 39;
pub const ENOSYS: PosixErrorNumber = 40;
pub const ENOTEMPTY: PosixErrorNumber = 41;
pub const EILSEQ: PosixErrorNumber = 42;
