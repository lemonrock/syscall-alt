// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::PosixErrorNumber;


pub const E2BIG: PosixErrorNumber = 7;
pub const EACCES: PosixErrorNumber = 13;
pub const EADDRINUSE: PosixErrorNumber = 98;
pub const EADDRNOTAVAIL: PosixErrorNumber = 99;
pub const EADV: PosixErrorNumber = 68;
pub const EAFNOSUPPORT: PosixErrorNumber = 97;
pub const EAGAIN: PosixErrorNumber = 11;
pub const EALREADY: PosixErrorNumber = 114;
pub const EBADE: PosixErrorNumber = 52;
pub const EBADF: PosixErrorNumber = 9;
pub const EBADFD: PosixErrorNumber = 77;
pub const EBADMSG: PosixErrorNumber = 74;
pub const EBADR: PosixErrorNumber = 53;
pub const EBADRQC: PosixErrorNumber = 56;
pub const EBADSLT: PosixErrorNumber = 57;
pub const EBFONT: PosixErrorNumber = 59;
pub const EBUSY: PosixErrorNumber = 16;
pub const ECANCELED: PosixErrorNumber = 125;
pub const ECHILD: PosixErrorNumber = 10;
pub const ECHRNG: PosixErrorNumber = 44;
pub const ECOMM: PosixErrorNumber = 70;
pub const ECONNABORTED: PosixErrorNumber = 103;
pub const ECONNREFUSED: PosixErrorNumber = 111;
pub const ECONNRESET: PosixErrorNumber = 104;
pub const EDEADLK: PosixErrorNumber = 35;
pub const EDEADLOCK: PosixErrorNumber = 58;
pub const EDESTADDRREQ: PosixErrorNumber = 89;
pub const EDOM: PosixErrorNumber = 33;
pub const EDOTDOT: PosixErrorNumber = 73;
pub const EDQUOT: PosixErrorNumber = 122;
pub const EEXIST: PosixErrorNumber = 17;
pub const EFAULT: PosixErrorNumber = 14;
pub const EFBIG: PosixErrorNumber = 27;
pub const EHOSTDOWN: PosixErrorNumber = 112;
pub const EHOSTUNREACH: PosixErrorNumber = 113;
pub const EHWPOISON: PosixErrorNumber = 133;
pub const EIDRM: PosixErrorNumber = 43;
pub const EILSEQ: PosixErrorNumber = 84;
pub const EINPROGRESS: PosixErrorNumber = 115;
pub const EINTR: PosixErrorNumber = 4;
pub const EINVAL: PosixErrorNumber = 22;
pub const EIO: PosixErrorNumber = 5;
pub const EISCONN: PosixErrorNumber = 106;
pub const EISDIR: PosixErrorNumber = 21;
pub const EISNAM: PosixErrorNumber = 120;
pub const EKEYEXPIRED: PosixErrorNumber = 127;
pub const EKEYREJECTED: PosixErrorNumber = 129;
pub const EKEYREVOKED: PosixErrorNumber = 128;
pub const EL2HLT: PosixErrorNumber = 51;
pub const EL2NSYNC: PosixErrorNumber = 45;
pub const EL3HLT: PosixErrorNumber = 46;
pub const EL3RST: PosixErrorNumber = 47;
pub const ELIBACC: PosixErrorNumber = 79;
pub const ELIBBAD: PosixErrorNumber = 80;
pub const ELIBEXEC: PosixErrorNumber = 83;
pub const ELIBMAX: PosixErrorNumber = 82;
pub const ELIBSCN: PosixErrorNumber = 81;
pub const ELNRNG: PosixErrorNumber = 48;
pub const ELOOP: PosixErrorNumber = 40;
pub const EMEDIUMTYPE: PosixErrorNumber = 124;
pub const EMFILE: PosixErrorNumber = 24;
pub const EMLINK: PosixErrorNumber = 31;
pub const EMSGSIZE: PosixErrorNumber = 90;
pub const EMULTIHOP: PosixErrorNumber = 72;
pub const ENAMETOOLONG: PosixErrorNumber = 36;
pub const ENAVAIL: PosixErrorNumber = 119;
pub const ENETDOWN: PosixErrorNumber = 100;
pub const ENETRESET: PosixErrorNumber = 102;
pub const ENETUNREACH: PosixErrorNumber = 101;
pub const ENFILE: PosixErrorNumber = 23;
pub const ENOANO: PosixErrorNumber = 55;
pub const ENOBUFS: PosixErrorNumber = 105;
pub const ENOCSI: PosixErrorNumber = 50;
pub const ENODATA: PosixErrorNumber = 61;
pub const ENODEV: PosixErrorNumber = 19;
pub const ENOENT: PosixErrorNumber = 2;
pub const ENOEXEC: PosixErrorNumber = 8;
pub const ENOKEY: PosixErrorNumber = 126;
pub const ENOLCK: PosixErrorNumber = 37;
pub const ENOLINK: PosixErrorNumber = 67;
pub const ENOMEDIUM: PosixErrorNumber = 123;
pub const ENOMEM: PosixErrorNumber = 12;
pub const ENOMSG: PosixErrorNumber = 42;
pub const ENONET: PosixErrorNumber = 64;
pub const ENOPKG: PosixErrorNumber = 65;
pub const ENOPROTOOPT: PosixErrorNumber = 92;
pub const ENOSPC: PosixErrorNumber = 28;
pub const ENOSR: PosixErrorNumber = 63;
pub const ENOSTR: PosixErrorNumber = 60;
pub const ENOSYS: PosixErrorNumber = 38;
pub const ENOTBLK: PosixErrorNumber = 15;
pub const ENOTCONN: PosixErrorNumber = 107;
pub const ENOTDIR: PosixErrorNumber = 20;
pub const ENOTEMPTY: PosixErrorNumber = 39;
pub const ENOTNAM: PosixErrorNumber = 118;
pub const ENOTRECOVERABLE: PosixErrorNumber = 131;
pub const ENOTSOCK: PosixErrorNumber = 88;
pub const ENOTSUP: PosixErrorNumber = EOPNOTSUPP;
pub const ENOTTY: PosixErrorNumber = 25;
pub const ENOTUNIQ: PosixErrorNumber = 76;
pub const ENXIO: PosixErrorNumber = 6;
pub const EOPNOTSUPP: PosixErrorNumber = 95;
pub const EOVERFLOW: PosixErrorNumber = 75;
pub const EOWNERDEAD: PosixErrorNumber = 130;
pub const EPERM: PosixErrorNumber = 1;
pub const EPFNOSUPPORT: PosixErrorNumber = 96;
pub const EPIPE: PosixErrorNumber = 32;
pub const EPROTO: PosixErrorNumber = 71;
pub const EPROTONOSUPPORT: PosixErrorNumber = 93;
pub const EPROTOTYPE: PosixErrorNumber = 91;
pub const ERANGE: PosixErrorNumber = 34;
pub const EREMCHG: PosixErrorNumber = 78;
pub const EREMOTE: PosixErrorNumber = 66;
pub const EREMOTEIO: PosixErrorNumber = 121;
pub const ERESTART: PosixErrorNumber = 85;
pub const ERFKILL: PosixErrorNumber = 132;
pub const EROFS: PosixErrorNumber = 30;
pub const ESHUTDOWN: PosixErrorNumber = 108;
pub const ESOCKTNOSUPPORT: PosixErrorNumber = 94;
pub const ESPIPE: PosixErrorNumber = 29;
pub const ESRCH: PosixErrorNumber = 3;
pub const ESRMNT: PosixErrorNumber = 69;
pub const ESTALE: PosixErrorNumber = 116;
pub const ESTRPIPE: PosixErrorNumber = 86;
pub const ETIME: PosixErrorNumber = 62;
pub const ETIMEDOUT: PosixErrorNumber = 110;
pub const ETOOMANYREFS: PosixErrorNumber = 109;
pub const ETXTBSY: PosixErrorNumber = 26;
pub const EUCLEAN: PosixErrorNumber = 117;
pub const EUNATCH: PosixErrorNumber = 49;
pub const EUSERS: PosixErrorNumber = 87;
pub const EWOULDBLOCK: PosixErrorNumber = EAGAIN;
pub const EXDEV: PosixErrorNumber = 18;
pub const EXFULL: PosixErrorNumber = 54;
