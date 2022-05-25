use super::Alignment;
use cfg_if::cfg_if;

/// Alignment to a SIMD block guarantee.
///
/// It is guaranteed that this alignment's [`size`](`Alignment::size`) is a multiplicity
/// of the size of a SIMD register of the target architecture.
///
/// # Alignments
///
/// The alignment size will be the first entry in the below table
/// that is supported by the target CPU, as long as the application
/// is compiled with the appropriate [target feature](https://doc.rust-lang.org/reference/conditional-compilation.html#target_feature).
///
/// | CPU feature     | Alignment (bytes) |
/// |-----------------|------------------:|
/// | AVX2            | 32                |
/// | SSE             | 16                |
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
pub struct SimdBlock {}

/// Alignment to two SIMD blocks guarantee.
///
/// This size is always equal to twice the size of [`SimdBlock`].
///
/// # Examples
#[cfg_attr(not(feature = "simd"), doc = "```ignore")]
#[cfg_attr(feature = "simd", doc = "```")]
/// use aligners::alignment::{self, Alignment};
///
/// assert_eq!(2 * alignment::SimdBlock::size(), alignment::TwoSimdBlocks::size());
/// ```
#[derive(Debug)]
#[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
pub struct TwoSimdBlocks {}

// SAFETY:
// Always returning a const value that is a power of two.
unsafe impl Alignment for SimdBlock {
    #[inline(always)]
    fn size() -> usize {
        cfg_if! {
            if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                cfg_if! {
                    if #[cfg(target_feature = "avx2")] {
                        32
                    }
                    else if #[cfg(target_feature = "sse")] {
                        16
                    }
                }
            } else if #[cfg(doc)] {
                32
            }
            else {
                compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
                unreachable!();
            }
        }
    }
}

// SAFETY:
// Safe as long as the impl for `SimdBlock` is safe, since we multiply by 2.
unsafe impl Alignment for TwoSimdBlocks {
    #[inline(always)]
    fn size() -> usize {
        SimdBlock::size() * 2
    }
}
