#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use std::arch::is_x86_feature_detected;
pub trait Tzcnt {
    fn tzcnt(self) -> u64;
}
/*#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "bmi1")]
#[inline(always)]*/

macro_rules! impl_tzcnt {
    ($id:ident) => {
        impl Tzcnt for $id {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            #[target_feature(enable = "bmi1")]
            #[inline]
            fn tzcnt(self: u64) -> u64 {
                core::arch::x86_64::_tzcnt_u64(x) as u64
            }
            #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
            #[inline]
            fn tzcnt(self:u64) -> u64 {
                self.trailing_zeros() as u64
            }
        }
    };
}

/// Resets lowest set bit.
pub trait Blsr {
    fn blsr(self) -> u64;
}

macro_rules! impl_blsr {
    ($id:ident) => {
        impl Blsr for $id {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            #[target_feature(enable = "bmi1")]
            #[inline]
            fn blsr(self: u64) -> u64 {
                core::arch::x86_64::_blsr_u64(self)
            }
            #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
            #[inline]
            fn blsr(self:u64) -> u64 {
                self & (self.wrapping_sub(1))
            }
            
        }
    };
}

pub trait Popcnt {
    fn popcnt(self) -> u64;
}

macro_rules! impl_popcnt {
    ($id:ident) => {
        impl Popcnt for $id {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            #[target_feature(enable = "bmi1")]
            #[inline]
            fn popcnt(self: u64) -> u64 {
                core::arch::x86_64::_popcnt64(self as i64)
            }
            #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
            #[inline]
            fn popcnt(self:u64) -> u64 {
                self.count_ones() as u64
            }
        }
    };
}
macro_rules! impl_all {
    ($impl_macro:ident: $($id:ident),*) => {
        $(
            $impl_macro!($id);
        )*
    }
}

impl_all!(impl_tzcnt: u64);
impl_all!(impl_popcnt: u64);
impl_all!(impl_blsr: u64);
