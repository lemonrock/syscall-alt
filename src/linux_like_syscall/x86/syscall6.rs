// This file is part of syscall-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT. No part of syscall-alt, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syscall-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syscall-alt/master/COPYRIGHT.

// This code is derived from original work by the syscall-rs project; it is an extract from file src/platform/linux-x86/mod.rs
// Whilst the layout and naming has changed, the fundamental approach is the same. Copyright for original code follows (The "COPYRIGHT file at the top-level directory of this distribution" referred to is embedded inside our COPYRIGHT file found as in our copyright statement in line 1 of ths file); we have chosen to license (sic) it under the MIT License:-

// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

#[inline(always)]
pub unsafe fn syscall6(n: SyscallNumber, a: SyscallArgument, b: SyscallArgument, c: SyscallArgument, d: SyscallArgument, e: SyscallArgument, f: SyscallArgument) -> SyscallResult
{
	let result: SyscallResult;
	let inputOperands = [n, a, b, c, d, e, f];
	asm!
	(
		"
			push %ebp
			movl 24(%eax), %ebp
			movl 20(%eax), %edi
			movl 16(%eax), %esi
			movl 12(%eax), %edx
			movl  8(%eax), %ecx
			movl  4(%eax), %ebx
			movl  0(%eax), %eax
			int $$0x80
			pop %ebp
		"
		: "={eax}"(result)
		: "{eax}"(inputOperands)
		: "memory" "cc" "ebx" "ecx" "edx" "esi" "edi" "ebp"
		: "volatile"
	);
	result
}
