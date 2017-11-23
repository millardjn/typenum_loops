//! #Example
//!
//! ```
//! extern crate typenum;
//! extern crate typenum_loops;
//!
//! use typenum::{U4, U6};
//! use typenum_loops::Loop;
//!
//! fn main(){
//!     let arr: &mut[usize] = &mut[0; 4];
//!     // for i in 0..4 {arr[i] = i} fully unrolled by 4
//!     U4::full_unroll(&mut |i| arr[i] = i);
//!
//!     let arr2: &mut[usize] = &mut[0; 13];
//!     // for i in 0..13 {arr2[i] = i} unrolled by 6
//!     U6::partial_unroll(13, &mut |i, _| arr2[i] = i);
//! }
//! ```


extern crate typenum;
use typenum::UInt;
use typenum::UTerm;
use typenum::marker_traits::Unsigned;
use std::ops::Sub;
use typenum::bit::*;
use typenum::operator_aliases::Sub1;

pub trait Loop: Unsigned {

	/// Call `f` for `0..N` in a fully unrolled loop
	///
	/// where `N = Self::to_usize()` of the unsigned typenum type.
	///
	/// * `f` `FnMut(usize)` to be called in the loop.
	///
	/// The usize passed to `f` is the iteration number starting from 0.
	#[inline(always)]
	fn full_unroll<F: FnMut(usize)>(f: &mut F){
		// for i in 0..Self::to_usize() {
		// 	f(i);
		// }
		Self::_recurse(f);
	}

	/// Call `f` for `0..k` in a loop unrolled by a factor of N,
	///
	/// where `N = <Self as Unsigned>::to_usize()`.
	/// An edge loop handles the `k%N !=0` case.
	///
	/// * `k` `usize`, size of loop
	/// * `f` `FnMut(usize, usize)` to be called in the loop.
	///
	/// The first usize passed to `f` is the iteration number starting from 0.
	/// The second usize passed to `f` is the unroll number (iteration number % N)
	#[inline(always)]
	fn partial_unroll<F: FnMut(usize, usize)>(k: usize, f: &mut F){
		let n = Self::to_usize();
		for i in 0..k/n{
			let mut r = 0;
			Self::full_unroll(&mut |j|{ f.call_inline(j + n*i, r); r +=1;});
			//Self::full_unroll(|j|{ call_fn_with2(&mut f, j + n*i, r); r +=1;});
		}

		let mut r = 0;
		for i in (k/n)*n..k{
			f.call_inline(i, r);
			//call_fn_with2(&mut f, i, r);
			r +=1;
		}
	}

	
	/// Don't use. Utility method implemented for all `Unsigned` type nums. `i` should always be Self::to_usize() on outer call.
	#[inline(always)]
	fn _recurse<F: FnMut(usize)>(f: &mut F);
}

trait InlineCall {
	#[inline(always)]
	fn call_inline(&mut self, i: usize);
}

impl<F: FnMut(usize)> InlineCall for F {
	#[inline(always)]
	fn call_inline(&mut self, i: usize){
		self(i);
	}
}

trait DualInlineCall {
	#[inline(always)]
	fn call_inline(&mut self, i: usize, j: usize);
}

impl<F: FnMut(usize, usize)> DualInlineCall for F {
	#[inline(always)]
	fn call_inline(&mut self, i: usize, j: usize){
		self(i, j);
	}
}

impl<U: Unsigned, B: Bit, C: Bit> Loop for UInt<UInt<U, B>, C> where UInt<UInt<U, B>, C>: Sub<B1>, Sub1<UInt<UInt<U, B>, C>>: Loop {
	#[inline(always)]
	fn _recurse<F: FnMut(usize)>(f: &mut F){
		//call_fn_with1(&mut f, i - Self::to_usize());
		//f(i - Self::to_usize());
		<Sub1<Self>>::_recurse(f);
		f.call_inline(Self::to_usize()-1);
	}
}

impl Loop for UInt<UTerm, B1> {
	#[inline(always)]
	fn _recurse<F: FnMut(usize)>(f: &mut F){
		f.call_inline(Self::to_usize()-1);
		//f.call_inline(i - Self::to_usize());
		//f(i - Self::to_usize());
		//call_fn_with1(&mut f, i - Self::to_usize());
	}
}


impl Loop for UTerm{
	#[inline(always)]
	fn _recurse<F: FnMut(usize)>(_f: &mut F){}
}


#[cfg(test)]
mod tests {
	use typenum::U4;
	use Loop;

	#[test]
	fn test_loop() {
		let vec: Vec<usize> = (0..4).collect();
		let arr: &mut[usize] = &mut[0; 4];

		U4::full_unroll(&mut |i| arr[i] += i);

		assert_eq!(arr, vec.as_slice());
	}


	#[test]
	fn test_unroll() {
		let expected1: Vec<usize> = (0..13).collect();
		let arr1: &mut[usize] = &mut[0; 13];

		let expected2: Vec<usize> = (0..13).map(|i| i%4).collect();
		let arr2: &mut[usize] = &mut[0; 13];

		U4::partial_unroll(13, &mut |i, j| {
			arr1[i] += i;
			arr2[i] = j;
		});

		assert_eq!(arr1, expected1.as_slice());
		assert_eq!(arr2, expected2.as_slice());
	}
}
