// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::constants::linux_like::SYS::*;

// Strictly speaking, isize is incorrect. We want SyscallNumber
// But we are not allowed to use that as the required RFC was closed.
// isize currently tracks c_long size and signedness in all current libc
// configurations, but libc actually uses either i32 or i64 for c_long
#[repr(isize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum Syscall
{
	#[cfg(not(target_arch = "x86"))] accept = SYS_accept,
	accept4 = SYS_accept4,
	acct = SYS_acct,
	add_key = SYS_add_key,
	adjtimex = SYS_adjtimex,
	bind = SYS_bind,
	bpf = SYS_bpf,
	brk = SYS_brk,
	capget = SYS_capget,
	capset = SYS_capset,
	chdir = SYS_chdir,
	chroot = SYS_chroot,
	clock_adjtime = SYS_clock_adjtime,
	clock_getres = SYS_clock_getres,
	clock_gettime = SYS_clock_gettime,
	clock_nanosleep = SYS_clock_nanosleep,
	clock_settime = SYS_clock_settime,
	clone = SYS_clone,
	close = SYS_close,
	connect = SYS_connect,
	copy_file_range = SYS_copy_file_range,
	delete_module = SYS_delete_module,
	dup = SYS_dup,
	dup3 = SYS_dup3,
	epoll_create1 = SYS_epoll_create1,
	epoll_ctl = SYS_epoll_ctl,
	epoll_pwait = SYS_epoll_pwait,
	eventfd2 = SYS_eventfd2,
	execve = SYS_execve,
	execveat = SYS_execveat,
	exit = SYS_exit,
	exit_group = SYS_exit_group,
	faccessat = SYS_faccessat,
	#[cfg(not(target_arch = "mips"))] fadvise64 = SYS_fadvise64,
	fallocate = SYS_fallocate,
	fanotify_init = SYS_fanotify_init,
	fanotify_mark = SYS_fanotify_mark,
	fchdir = SYS_fchdir,
	fchmod = SYS_fchmod,
	fchmodat = SYS_fchmodat,
	fchown = SYS_fchown,
	fchownat = SYS_fchownat,
	fcntl = SYS_fcntl,
	fdatasync = SYS_fdatasync,
	fgetxattr = SYS_fgetxattr,
	finit_module = SYS_finit_module,
	flistxattr = SYS_flistxattr,
	flock = SYS_flock,
	fremovexattr = SYS_fremovexattr,
	fsetxattr = SYS_fsetxattr,
	// fstat = SYS_fstat, // There are at least 3 variants of this on x86, and the mapping is not clear
	#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))] fstatat = SYS_fstatat,
	fstatfs = SYS_fstatfs,
	fsync = SYS_fsync,
	ftruncate = SYS_ftruncate,
	futex = SYS_futex,
	getcpu = SYS_getcpu,
	getcwd = SYS_getcwd,
	getdents64 = SYS_getdents64,
	getegid = SYS_getegid,
	geteuid = SYS_geteuid,
	getgid = SYS_getgid,
	getgroups = SYS_getgroups,
	getitimer = SYS_getitimer,
	getpeername = SYS_getpeername,
	getpgid = SYS_getpgid,
	getpid = SYS_getpid,
	getppid = SYS_getppid,
	getpriority = SYS_getpriority,
	getrandom = SYS_getrandom,
	getresgid = SYS_getresgid,
	getresuid = SYS_getresuid,
	getrlimit = SYS_getrlimit,
	getrusage = SYS_getrusage,
	getsid = SYS_getsid,
	getsockname = SYS_getsockname,
	getsockopt = SYS_getsockopt,
	gettid = SYS_gettid,
	gettimeofday = SYS_gettimeofday,
	getuid = SYS_getuid,
	getxattr = SYS_getxattr,
	get_mempolicy = SYS_get_mempolicy,
	get_robust_list = SYS_get_robust_list,
	init_module = SYS_init_module,
	inotify_add_watch = SYS_inotify_add_watch,
	inotify_init1 = SYS_inotify_init1,
	inotify_rm_watch = SYS_inotify_rm_watch,
	ioctl = SYS_ioctl,
	ioprio_get = SYS_ioprio_get,
	ioprio_set = SYS_ioprio_set,
	io_cancel = SYS_io_cancel,
	io_destroy = SYS_io_destroy,
	io_getevents = SYS_io_getevents,
	io_setup = SYS_io_setup,
	io_submit = SYS_io_submit,
	kcmp = SYS_kcmp,
	kexec_load = SYS_kexec_load,
	keyctl = SYS_keyctl,
	kill = SYS_kill,
	lgetxattr = SYS_lgetxattr,
	linkat = SYS_linkat,
	listen = SYS_listen,
	listxattr = SYS_listxattr,
	llistxattr = SYS_llistxattr,
	lookup_dcookie = SYS_lookup_dcookie,
	lremovexattr = SYS_lremovexattr,
	lseek = SYS_lseek,
	lsetxattr = SYS_lsetxattr,
	madvise = SYS_madvise,
	mbind = SYS_mbind,
	membarrier = SYS_membarrier,
	memfd_create = SYS_memfd_create,
	migrate_pages = SYS_migrate_pages,
	mincore = SYS_mincore,
	mkdirat = SYS_mkdirat,
	mknodat = SYS_mknodat,
	mlock = SYS_mlock,
	mlock2 = SYS_mlock2,
	mlockall = SYS_mlockall,
	mmap = SYS_mmap,
	mount = SYS_mount,
	move_pages = SYS_move_pages,
	mprotect = SYS_mprotect,
	mq_getsetattr = SYS_mq_getsetattr,
	mq_notify = SYS_mq_notify,
	mq_open = SYS_mq_open,
	mq_timedreceive = SYS_mq_timedreceive,
	mq_timedsend = SYS_mq_timedsend,
	mq_unlink = SYS_mq_unlink,
	mremap = SYS_mremap,
	#[cfg(not(target_arch = "x86"))] msgctl = SYS_msgctl,
	#[cfg(not(target_arch = "x86"))] msgget = SYS_msgget,
	#[cfg(not(target_arch = "x86"))] msgrcv = SYS_msgrcv,
	#[cfg(not(target_arch = "x86"))] msgsnd = SYS_msgsnd,
	msync = SYS_msync,
	munlock = SYS_munlock,
	munlockall = SYS_munlockall,
	munmap = SYS_munmap,
	name_to_handle_at = SYS_name_to_handle_at,
	nanosleep = SYS_nanosleep,
	nfsservctl = SYS_nfsservctl,
	openat = SYS_openat,
	open_by_handle_at = SYS_open_by_handle_at,
	#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))] or1k_atomic = SYS_or1k_atomic,
	perf_event_open = SYS_perf_event_open,
	personality = SYS_personality,
	pipe2 = SYS_pipe2,
	pivot_root = SYS_pivot_root,
	ppoll = SYS_ppoll,
	prctl = SYS_prctl,
	pread64 = SYS_pread64,
	preadv = SYS_preadv,
	preadv2 = SYS_preadv2,
	prlimit64 = SYS_prlimit64,
	process_vm_readv = SYS_process_vm_readv,
	process_vm_writev = SYS_process_vm_writev,
	pselect6 = SYS_pselect6,
	ptrace = SYS_ptrace,
	pwrite64 = SYS_pwrite64,
	pwritev = SYS_pwritev,
	pwritev2 = SYS_pwritev2,
	quotactl = SYS_quotactl,
	read = SYS_read,
	readahead = SYS_readahead,
	readlinkat = SYS_readlinkat,
	readv = SYS_readv,
	reboot = SYS_reboot,
	recvfrom = SYS_recvfrom,
	recvmmsg = SYS_recvmmsg,
	recvmsg = SYS_recvmsg,
	remap_file_pages = SYS_remap_file_pages,
	removexattr = SYS_removexattr,
	renameat = SYS_renameat,
	renameat2 = SYS_renameat2,
	request_key = SYS_request_key,
	restart_syscall = SYS_restart_syscall,
	rt_sigaction = SYS_rt_sigaction,
	rt_sigpending = SYS_rt_sigpending,
	rt_sigprocmask = SYS_rt_sigprocmask,
	rt_sigqueueinfo = SYS_rt_sigqueueinfo,
	rt_sigreturn = SYS_rt_sigreturn,
	rt_sigsuspend = SYS_rt_sigsuspend,
	rt_sigtimedwait = SYS_rt_sigtimedwait,
	rt_tgsigqueueinfo = SYS_rt_tgsigqueueinfo,
	sched_getaffinity = SYS_sched_getaffinity,
	sched_getattr = SYS_sched_getattr,
	sched_getparam = SYS_sched_getparam,
	sched_getscheduler = SYS_sched_getscheduler,
	sched_get_priority_max = SYS_sched_get_priority_max,
	sched_get_priority_min = SYS_sched_get_priority_min,
	sched_rr_get_interval = SYS_sched_rr_get_interval,
	sched_setaffinity = SYS_sched_setaffinity,
	sched_setattr = SYS_sched_setattr,
	sched_setparam = SYS_sched_setparam,
	sched_setscheduler = SYS_sched_setscheduler,
	sched_yield = SYS_sched_yield,
	seccomp = SYS_seccomp,
	#[cfg(not(target_arch = "x86"))] semctl = SYS_semctl,
	#[cfg(not(target_arch = "x86"))] semget = SYS_semget,
	#[cfg(not(target_arch = "x86"))] semop = SYS_semop,
	#[cfg(not(target_arch = "x86"))] semtimedop = SYS_semtimedop,
	sendfile = SYS_sendfile,
	sendmmsg = SYS_sendmmsg,
	sendmsg = SYS_sendmsg,
	sendto = SYS_sendto,
	setdomainname = SYS_setdomainname,
	setfsgid = SYS_setfsgid,
	setfsuid = SYS_setfsuid,
	setgid = SYS_setgid,
	setgroups = SYS_setgroups,
	sethostname = SYS_sethostname,
	setitimer = SYS_setitimer,
	setns = SYS_setns,
	setpgid = SYS_setpgid,
	setpriority = SYS_setpriority,
	setregid = SYS_setregid,
	setresgid = SYS_setresgid,
	setresuid = SYS_setresuid,
	setreuid = SYS_setreuid,
	setrlimit = SYS_setrlimit,
	setsid = SYS_setsid,
	setsockopt = SYS_setsockopt,
	settimeofday = SYS_settimeofday,
	setuid = SYS_setuid,
	setxattr = SYS_setxattr,
	set_mempolicy = SYS_set_mempolicy,
	set_robust_list = SYS_set_robust_list,
	set_tid_address = SYS_set_tid_address,
	#[cfg(not(target_arch = "x86"))] shmat = SYS_shmat,
	#[cfg(not(target_arch = "x86"))] shmctl = SYS_shmctl,
	#[cfg(not(target_arch = "x86"))] shmdt = SYS_shmdt,
	#[cfg(not(target_arch = "x86"))] shmget = SYS_shmget,
	shutdown = SYS_shutdown,
	sigaltstack = SYS_sigaltstack,
	signalfd4 = SYS_signalfd4,
	socket = SYS_socket,
	socketpair = SYS_socketpair,
	splice = SYS_splice,
	statfs = SYS_statfs,
	swapoff = SYS_swapoff,
	swapon = SYS_swapon,
	symlinkat = SYS_symlinkat,
	sync = SYS_sync,
	syncfs = SYS_syncfs,
	sync_file_range = SYS_sync_file_range,
	sysinfo = SYS_sysinfo,
	syslog = SYS_syslog,
	tee = SYS_tee,
	tgkill = SYS_tgkill,
	timerfd_create = SYS_timerfd_create,
	timerfd_gettime = SYS_timerfd_gettime,
	timerfd_settime = SYS_timerfd_settime,
	timer_create = SYS_timer_create,
	timer_delete = SYS_timer_delete,
	timer_getoverrun = SYS_timer_getoverrun,
	timer_gettime = SYS_timer_gettime,
	timer_settime = SYS_timer_settime,
	times = SYS_times,
	tkill = SYS_tkill,
	truncate = SYS_truncate,
	umask = SYS_umask,
	umount2 = SYS_umount2,
	uname = SYS_uname,
	unlinkat = SYS_unlinkat,
	unshare = SYS_unshare,
	userfaultfd = SYS_userfaultfd,
	utimensat = SYS_utimensat,
	vhangup = SYS_vhangup,
	vmsplice = SYS_vmsplice,
	wait4 = SYS_wait4,
	waitid = SYS_waitid,
	write = SYS_write,
	writev = SYS_writev,
}

impl Syscall
{
	#[inline(always)]
	pub unsafe fn syscall0(self) -> ::SyscallResult
	{
		syscalls::syscall0(self as SyscallNumber)
	}

	#[inline(always)]
	pub unsafe fn syscall1(self, a: SyscallArgument) -> ::SyscallResult
	{
		syscalls::syscall1(self as SyscallNumber, a)
	}

	#[inline(always)]
	pub unsafe fn syscall2(self, a: SyscallArgument, b: SyscallArgument) -> ::SyscallResult
	{
		syscalls::syscall2(self as SyscallNumber, a, b)
	}

	#[inline(always)]
	pub unsafe fn syscall3(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument) -> ::SyscallResult
	{
		syscalls::syscall3(self as SyscallNumber, a, b, c)
	}

	#[inline(always)]
	pub unsafe fn syscall4(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument) -> ::SyscallResult
	{
		syscalls::syscall4(self as SyscallNumber, a, b, c, d)
	}

	#[inline(always)]
	pub unsafe fn syscall5(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument, e: SyscallArgument) -> ::SyscallResult
	{
		syscalls::syscall5(self as SyscallNumber, a, b, c, d, e)
	}
	
	#[inline(always)]
	pub unsafe fn syscall6(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument, e: SyscallArgument, f: SyscallArgument) -> ::SyscallResult
	{
		syscalls::syscall6(self as SyscallNumber, a, b, c, d, e, f)
	}
	
	#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
	#[inline(always)]
	pub unsafe fn syscall7(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument, e: SyscallArgument, f: SyscallArgument, g: SyscallArgument) -> ::SyscallResult
	{
		syscalls::syscall7(self as SyscallNumber, a, b, c, d, e, f, g)
	}
}
