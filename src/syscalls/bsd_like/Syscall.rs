// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


use ::constants::SYS::*;

// Strictly speaking, isize is incorrect. We want SyscallNumber
// But we are not allowed to use that as the required RFC was closed.
#[repr(isize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum Syscall
{
	#[cfg(not(any(target_os = "ios", target_os = "macos")))] accept4 = SYS_accept4,
	close = SYS_close,
	connect = SYS_connect,
	getsockname = SYS_getsockname,
	getsockopt = SYS_getsockopt,
	readv = SYS_readv,
	recvfrom = SYS_recvfrom,
	recvmmsg = SYS_recvmmsg,
	recvmsg = SYS_recvmsg,
	sendfile = SYS_sendfile,
	sendmmsg = SYS_sendmmsg,
	sendmsg = SYS_sendmsg,
	sendto = SYS_sendto,
	setsockopt = SYS_setsockopt,
	shutdown = SYS_shutdown,
	socket = SYS_socket,
	socketpair = SYS_socketpair,
	write = SYS_write,
	writev = SYS_writev,
}

impl Syscall
{
	#[inline(always)]
	pub unsafe fn syscall0(self) -> SyscallResult
	{
		::syscalls::syscall0(self as SyscallNumber)
	}

	#[inline(always)]
	pub unsafe fn syscall1(self, a: SyscallArgument) -> SyscallResult
	{
		::syscalls::syscall1(self as SyscallNumber, a)
	}

	#[inline(always)]
	pub unsafe fn syscall2(self, a: SyscallArgument, b: SyscallArgument) -> SyscallResult
	{
		::syscalls::syscall2(self as SyscallNumber, a, b)
	}

	#[inline(always)]
	pub unsafe fn syscall3(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument) -> SyscallResult
	{
		::syscalls::syscall3(self as SyscallNumber, a, b, c)
	}

	#[inline(always)]
	pub unsafe fn syscall4(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument) -> SyscallResult
	{
		::syscalls::syscall4(self as SyscallNumber, a, b, c, d)
	}

	#[inline(always)]
	pub unsafe fn syscall5(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument, e: SyscallArgument) -> SyscallResult
	{
		::syscalls::syscall5(self as SyscallNumber, a, b, c, d, e)
	}
	
	#[inline(always)]
	pub unsafe fn syscall6(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument, e: SyscallArgument, f: SyscallArgument) -> SyscallResult
	{
		::syscalls::syscall6(self as SyscallNumber, a, b, c, d, e, f)
	}
	
	#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
	#[inline(always)]
	pub unsafe fn syscall7(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument, e: SyscallArgument, f: SyscallArgument, g: SyscallArgument) -> SyscallResult
	{
		::syscalls::syscall7(self as SyscallNumber, a, b, c, d, e, f, g)
	}
}
