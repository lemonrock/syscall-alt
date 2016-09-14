// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.


#[inline(always)]
pub unsafe fn syscall3(syscallNumber: SyscallNumber, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument) -> SyscallResult
{
	let result: SyscallResult;
	asm!
	(
		"svc 0"
		: "={x0}"(result)
		: "{x8}"(syscallNumber), "{x0}"(a), "{x1}"(b), "{x2}"(c)
		: "memory" "cc"
		: "volatile"
	);
	result
}
