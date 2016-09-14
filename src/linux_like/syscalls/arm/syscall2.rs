// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


#[inline(always)]
pub unsafe fn syscall2(syscallNumber: SyscallNumber, a: SyscallArgument, b: SyscallArgument) -> SyscallResult
{
	let result: SyscallResult;
	asm!
	(
		"swi $$0"
		: "={r0}"(result)
		: "{r7}"(syscallNumber), "{r0}"(a), "{r1}"(b)
		: "memory" "cc"
		: "volatile"
	);
	result
}
