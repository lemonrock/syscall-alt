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
}

impl Syscall
{
	#[inline(always)]
	pub unsafe fn syscall0(self) -> SyscallResult
	{
		unimplemented!();
	}

	#[inline(always)]
	pub unsafe fn syscall1(self, a: SyscallArgument) -> SyscallResult
	{
		unimplemented!();
	}

	#[inline(always)]
	pub unsafe fn syscall2(self, a: SyscallArgument, b: SyscallArgument) -> SyscallResult
	{
		unimplemented!();
	}

	#[inline(always)]
	pub unsafe fn syscall3(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument) -> SyscallResult
	{
		unimplemented!();
	}

	#[inline(always)]
	pub unsafe fn syscall4(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument) -> SyscallResult
	{
		unimplemented!();
	}

	#[inline(always)]
	pub unsafe fn syscall5(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument, e: SyscallArgument) -> SyscallResult
	{
		unimplemented!();
	}
	
	#[inline(always)]
	pub unsafe fn syscall6(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument, e: SyscallArgument, f: SyscallArgument) -> SyscallResult
	{
		unimplemented!();
	}
	
	#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
	#[inline(always)]
	pub unsafe fn syscall7(self, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument, e: SyscallArgument, f: SyscallArgument, g: SyscallArgument) -> SyscallResult
	{
		unimplemented!();
	}
}
