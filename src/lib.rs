extern crate typenum;
use typenum::UInt;
use typenum::UTerm;
use typenum::marker_traits::Unsigned;
use std::ops::Sub;
use typenum::bit::*;
use typenum::operator_aliases::Sub1;

pub trait Loop: Unsigned {

	#[inline(always)]
	/// Utility method. i should always be Self::to_usize() on outer call.
	fn recurse<F: FnMut(usize)>(i: usize, f: F);

	#[inline(always)]
	/// Call f for 0..N in a fully unrolled loop,
	/// where ```N = Self::to_usize()``` of the unsigned typenum type.
	/// * 'f' FnMut(usize) to be called in the loop.
	fn full_unroll<F: FnMut(usize)>(f: F){
		Self::recurse(Self::to_usize(), f);
	}

	/// Call f for 0..k in a loop unrolled by a factor of N,
	/// where ```N = Self::to_usize()``` of the unsigned typenum type.
	/// An edge loop handles the ```k%N !=0``` case.
	/// * 'f' FnMut(usize) to be called in the loop.
	/// * 'k' usize, size of loop
	fn partial_unroll<F: FnMut(usize)>(k: usize, mut f: F){
		let n = Self::to_usize();
		for i in 0..k/n{
			Self::full_unroll(|j| f(j + n*i));
		}

		for i in (k/n)*n..k{
			f(i);
		}
	}
}


impl<U: Unsigned, B: Bit, C: Bit> Loop for UInt<UInt<U, B>, C> where UInt<UInt<U, B>, C>: Sub<B1>, Sub1<UInt<UInt<U, B>, C>>: Loop {
	#[inline(always)]
	fn recurse<F: FnMut(usize)>(i: usize, mut f: F){
		f(i - Self::to_usize());
		<Sub1<Self>>::recurse(i, f);
	}
}

impl Loop for UInt<UTerm, B1> {
	#[inline(always)]
	fn recurse<F: FnMut(usize)>(i: usize, mut f: F){
		f(i - Self::to_usize());
	}
}


impl Loop for UTerm{
	#[inline(always)]
	fn recurse<F: FnMut(usize)>(_i: usize, _f: F){}
}


#[cfg(test)]
mod tests {
	use typenum::U4;
	use Loop;

	#[test]
	fn test_loop() {
		let vec: Vec<usize> = (0..4).collect();
		let arr: &mut[usize] = &mut[0; 4];

		U4::full_unroll(|i| arr[i] += i);

		assert_eq!(arr, vec.as_slice());
	}


	#[test]
	fn test_unroll() {
		let vec: Vec<usize> = (0..13).collect();
		let arr: &mut[usize] = &mut[0; 13];

		U4::partial_unroll(13, |i| arr[i] += i);

		assert_eq!(arr, vec.as_slice());
	}
}
