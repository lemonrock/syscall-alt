// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::SyscallNumber;


// Originates from https://github.com/openbsd/src/blob/master/sys/sys/syscall.h (as of commit 2d337c4 4th September 2016)
pub const SYS_accept4: SyscallNumber = 93;
pub const SYS_accept: SyscallNumber = 30;
pub const SYS_access: SyscallNumber = 33;
pub const SYS_acct: SyscallNumber = 51;
pub const SYS_adjfreq: SyscallNumber = 305;
pub const SYS_adjtime: SyscallNumber = 140;
pub const SYS_bind: SyscallNumber = 104;
pub const SYS_break: SyscallNumber = 17;
pub const SYS_chdir: SyscallNumber = 12;
pub const SYS_chflags: SyscallNumber = 34;
pub const SYS_chflagsat: SyscallNumber = 107;
pub const SYS_chmod: SyscallNumber = 15;
pub const SYS_chown: SyscallNumber = 16;
pub const SYS_chroot: SyscallNumber = 61;
pub const SYS_clock_getres: SyscallNumber = 89;
pub const SYS_clock_gettime: SyscallNumber = 87;
pub const SYS_clock_settime: SyscallNumber = 88;
pub const SYS_close: SyscallNumber = 6;
pub const SYS_closefrom: SyscallNumber = 287;
pub const SYS_connect: SyscallNumber = 98;
pub const SYS_dup2: SyscallNumber = 90;
pub const SYS_dup3: SyscallNumber = 102;
pub const SYS_dup: SyscallNumber = 41;
pub const SYS_execve: SyscallNumber = 59;
pub const SYS_exit: SyscallNumber = 1;
pub const SYS_faccessat: SyscallNumber = 313;
pub const SYS_fchdir: SyscallNumber = 13;
pub const SYS_fchflags: SyscallNumber = 35;
pub const SYS_fchmod: SyscallNumber = 124;
pub const SYS_fchmodat: SyscallNumber = 314;
pub const SYS_fchown: SyscallNumber = 123;
pub const SYS_fchownat: SyscallNumber = 315;
pub const SYS_fcntl: SyscallNumber = 92;
pub const SYS_fhopen: SyscallNumber = 264;
pub const SYS_fhstat: SyscallNumber = 294;
pub const SYS_fhstatfs: SyscallNumber = 65;
pub const SYS_flock: SyscallNumber = 131;
pub const SYS_fork: SyscallNumber = 2;
pub const SYS_fpathconf: SyscallNumber = 192;
pub const SYS_fstat: SyscallNumber = 53;
pub const SYS_fstatat: SyscallNumber = 42;
pub const SYS_fstatfs: SyscallNumber = 64;
pub const SYS_fsync: SyscallNumber = 95;
pub const SYS_ftruncate: SyscallNumber = 201;
pub const SYS_futimens: SyscallNumber = 85;
pub const SYS_futimes: SyscallNumber = 77;
pub const SYS_getdents: SyscallNumber = 99;
pub const SYS_getdtablecount: SyscallNumber = 18;
pub const SYS_getegid: SyscallNumber = 43;
pub const SYS_getentropy: SyscallNumber = 7;
pub const SYS_geteuid: SyscallNumber = 25;
pub const SYS_getfh: SyscallNumber = 161;
pub const SYS_getfsstat: SyscallNumber = 62;
pub const SYS_getgid: SyscallNumber = 47;
pub const SYS_getgroups: SyscallNumber = 79;
pub const SYS_getitimer: SyscallNumber = 70;
pub const SYS_getlogin59: SyscallNumber = 49;
pub const SYS_getlogin_r: SyscallNumber = 141;
pub const SYS_getpeername: SyscallNumber = 31;
pub const SYS_getpgid: SyscallNumber = 207;
pub const SYS_getpgrp: SyscallNumber = 81;
pub const SYS_getpid: SyscallNumber = 20;
pub const SYS_getppid: SyscallNumber = 39;
pub const SYS_getpriority: SyscallNumber = 100;
pub const SYS_getresgid: SyscallNumber = 283;
pub const SYS_getresuid: SyscallNumber = 281;
pub const SYS_getrlimit: SyscallNumber = 194;
pub const SYS_getrtable: SyscallNumber = 311;
pub const SYS_getrusage: SyscallNumber = 19;
pub const SYS_getsid: SyscallNumber = 255;
pub const SYS_getsockname: SyscallNumber = 32;
pub const SYS_getsockopt: SyscallNumber = 118;
pub const SYS_getthrid: SyscallNumber = 299;
pub const SYS_gettimeofday: SyscallNumber = 67;
pub const SYS_getuid: SyscallNumber = 24;
pub const SYS_ioctl: SyscallNumber = 54;
pub const SYS_issetugid: SyscallNumber = 253;
pub const SYS_kbind: SyscallNumber = 86;
pub const SYS_kevent: SyscallNumber = 72;
pub const SYS_kill: SyscallNumber = 122;
pub const SYS_kqueue: SyscallNumber = 269;
pub const SYS_ktrace: SyscallNumber = 45;
pub const SYS_lchown: SyscallNumber = 254;
pub const SYS_link: SyscallNumber = 9;
pub const SYS_linkat: SyscallNumber = 317;
pub const SYS_listen: SyscallNumber = 106;
pub const SYS_lseek: SyscallNumber = 199;
pub const SYS_lstat: SyscallNumber = 40;
pub const SYS_madvise: SyscallNumber = 75;
pub const SYS_mincore: SyscallNumber = 78;
pub const SYS_minherit: SyscallNumber = 250;
pub const SYS_mkdir: SyscallNumber = 136;
pub const SYS_mkdirat: SyscallNumber = 318;
pub const SYS_mkfifo: SyscallNumber = 132;
pub const SYS_mkfifoat: SyscallNumber = 319;
pub const SYS_mknod: SyscallNumber = 14;
pub const SYS_mknodat: SyscallNumber = 320;
pub const SYS_mlock: SyscallNumber = 203;
pub const SYS_mlockall: SyscallNumber = 271;
pub const SYS_mmap: SyscallNumber = 197;
pub const SYS_mount: SyscallNumber = 21;
pub const SYS_mprotect: SyscallNumber = 74;
pub const SYS_mquery: SyscallNumber = 286;
pub const SYS_msgctl: SyscallNumber = 297;
pub const SYS_msgget: SyscallNumber = 225;
pub const SYS_msgrcv: SyscallNumber = 227;
pub const SYS_msgsnd: SyscallNumber = 226;
pub const SYS_msync: SyscallNumber = 256;
pub const SYS_munlock: SyscallNumber = 204;
pub const SYS_munlockall: SyscallNumber = 272;
pub const SYS_munmap: SyscallNumber = 73;
pub const SYS_nanosleep: SyscallNumber = 91;
pub const SYS_nfssvc: SyscallNumber = 155;
pub const SYS_open: SyscallNumber = 5;
pub const SYS_openat: SyscallNumber = 321;
pub const SYS_pathconf: SyscallNumber = 191;
pub const SYS_pipe2: SyscallNumber = 101;
pub const SYS_pipe: SyscallNumber = 263;
pub const SYS_pledge: SyscallNumber = 108;
pub const SYS_poll: SyscallNumber = 252;
pub const SYS_ppoll: SyscallNumber = 109;
pub const SYS_pread: SyscallNumber = 173;
pub const SYS_preadv: SyscallNumber = 267;
pub const SYS_profil: SyscallNumber = 44;
pub const SYS_pselect: SyscallNumber = 110;
pub const SYS_ptrace: SyscallNumber = 26;
pub const SYS_pwrite: SyscallNumber = 174;
pub const SYS_pwritev: SyscallNumber = 268;
pub const SYS_quotactl: SyscallNumber = 148;
pub const SYS_read: SyscallNumber = 3;
pub const SYS_readlink: SyscallNumber = 58;
pub const SYS_readlinkat: SyscallNumber = 322;
pub const SYS_readv: SyscallNumber = 120;
pub const SYS_reboot: SyscallNumber = 55;
pub const SYS_recvfrom: SyscallNumber = 29;
pub const SYS_recvmsg: SyscallNumber = 27;
pub const SYS_rename: SyscallNumber = 128;
pub const SYS_renameat: SyscallNumber = 323;
pub const SYS_revoke: SyscallNumber = 56;
pub const SYS_rmdir: SyscallNumber = 137;
pub const SYS_sched_yield: SyscallNumber = 298;
pub const SYS_select: SyscallNumber = 71;
pub const SYS_semget: SyscallNumber = 221;
pub const SYS_semop: SyscallNumber = 290;
pub const SYS_sendmsg: SyscallNumber = 28;
pub const SYS_sendsyslog: SyscallNumber = 112;
pub const SYS_sendto: SyscallNumber = 133;
pub const SYS_setegid: SyscallNumber = 182;
pub const SYS_seteuid: SyscallNumber = 183;
pub const SYS_setgid: SyscallNumber = 181;
pub const SYS_setgroups: SyscallNumber = 80;
pub const SYS_setitimer: SyscallNumber = 69;
pub const SYS_setlogin: SyscallNumber = 50;
pub const SYS_setpgid: SyscallNumber = 82;
pub const SYS_setpriority: SyscallNumber = 96;
pub const SYS_setregid: SyscallNumber = 127;
pub const SYS_setresgid: SyscallNumber = 284;
pub const SYS_setresuid: SyscallNumber = 282;
pub const SYS_setreuid: SyscallNumber = 126;
pub const SYS_setrlimit: SyscallNumber = 195;
pub const SYS_setrtable: SyscallNumber = 310;
pub const SYS_setsid: SyscallNumber = 147;
pub const SYS_setsockopt: SyscallNumber = 105;
pub const SYS_settimeofday: SyscallNumber = 68;
pub const SYS_setuid: SyscallNumber = 23;
pub const SYS_shmat: SyscallNumber = 228;
pub const SYS_shmctl: SyscallNumber = 296;
pub const SYS_shmdt: SyscallNumber = 230;
pub const SYS_shmget: SyscallNumber = 289;
pub const SYS_shutdown: SyscallNumber = 134;
pub const SYS_sigaction: SyscallNumber = 46;
pub const SYS_sigaltstack: SyscallNumber = 288;
pub const SYS_sigpending: SyscallNumber = 52;
pub const SYS_sigprocmask: SyscallNumber = 48;
pub const SYS_sigreturn: SyscallNumber = 103;
pub const SYS_sigsuspend: SyscallNumber = 111;
pub const SYS_socket: SyscallNumber = 97;
pub const SYS_socketpair: SyscallNumber = 135;
pub const SYS_stat: SyscallNumber = 38;
pub const SYS_statfs: SyscallNumber = 63;
pub const SYS_swapctl: SyscallNumber = 193;
pub const SYS_symlink: SyscallNumber = 57;
pub const SYS_symlinkat: SyscallNumber = 324;
pub const SYS_sync: SyscallNumber = 36;
pub const SYS_sysarch: SyscallNumber = 165;
pub const SYS_syscall: SyscallNumber = 0;
pub const SYS_sysctl: SyscallNumber = 202;
pub const SYS_thrkill: SyscallNumber = 119;
pub const SYS_truncate: SyscallNumber = 200;
pub const SYS_umask: SyscallNumber = 60;
pub const SYS_unlink: SyscallNumber = 10;
pub const SYS_unlinkat: SyscallNumber = 325;
pub const SYS_unmount: SyscallNumber = 22;
pub const SYS_utimensat: SyscallNumber = 84;
pub const SYS_utimes: SyscallNumber = 76;
pub const SYS_utrace: SyscallNumber = 209;
pub const SYS_vfork: SyscallNumber = 66;
pub const SYS_wait4: SyscallNumber = 11;
pub const SYS_write: SyscallNumber = 4;
pub const SYS_writev: SyscallNumber = 121;
pub const SYS___getcwd: SyscallNumber = 304;
pub const SYS___get_tcb: SyscallNumber = 330;
pub const SYS___semctl: SyscallNumber = 295;
pub const SYS___set_tcb: SyscallNumber = 329;
pub const SYS___syscall: SyscallNumber = 198;
pub const SYS___tfork: SyscallNumber = 8;
pub const SYS___threxit: SyscallNumber = 302;
pub const SYS___thrsigdivert: SyscallNumber = 303;
pub const SYS___thrsleep: SyscallNumber = 94;
pub const SYS___thrwakeup: SyscallNumber = 301;
