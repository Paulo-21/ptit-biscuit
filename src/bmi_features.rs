use core::arch::x86_64::{_blsr_u64, _popcnt64, _tzcnt_u64};
pub trait Tzcnt {
    fn tzcnt(self) -> u64;
}

macro_rules! impl_tzcnt {
    ($id:ident) => {
        impl Tzcnt for $id {
            #[inline]
            fn tzcnt(self: u64) -> u64 {
                #[cfg(target_feature = "bmi1")]
                {
                    unsafe { _tzcnt_u64(self) }
                }
                #[cfg(not(target_feature = "bmi1"))]
                {
                    // otherwise, call a portable emulation of the BMI2 instruction
                    self.trailing_zeros() as u64
                }
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
            #[inline]
            fn blsr(self: u64) -> u64 {
                #[cfg(target_feature = "bmi1")]
                {
                    unsafe { _blsr_u64(self) }
                }
                #[cfg(not(target_feature = "bmi1"))]
                {
                    self & (self.wrapping_sub(1))
                }
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
            #[inline]
            fn popcnt(self: u64) -> u64 {
                #[cfg(target_feature = "bmi1")]
                {
                    unsafe { _popcnt64(self as i64) as u64 }
                }
                #[cfg(not(target_feature = "bmi1"))]
                {
                    self.count_ones() as u64
                }
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
