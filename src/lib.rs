#![feature(try_trait_v2)]

use std::convert::Infallible;
use std::ffi::CStr;
use std::num::NonZeroU32;
use std::ops::{ControlFlow, FromResidual, Try};
use std::result::Result;

// https://rust-lang.github.io/rfcs/3058-try-trait-v2.html#implementing-try-for-a-non-generic-type

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum CustomActionResult {
    Succeed = 0,
    Cancel = 1602,
    Fail = 1603,
}

impl Into<u32> for CustomActionResult {
    fn into(self) -> u32 {
        self as u32
    }
}

impl Into<i32> for CustomActionResult {
    fn into(self) -> i32 {
        self as i32
    }
}

pub struct CustomActionResultResidual(NonZeroU32);

impl Try for CustomActionResult {
    type Output = u32;
    type Residual = CustomActionResultResidual;

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Succeed => ControlFlow::Continue(0),
            _ => unsafe {
                ControlFlow::Break(CustomActionResultResidual(NonZeroU32::new_unchecked(
                    self as u32,
                )))
            },
        }
    }

    fn from_output(output: Self::Output) -> Self {
        match output {
            0 => CustomActionResult::Succeed,
            1602 => CustomActionResult::Cancel,
            _ => CustomActionResult::Fail,
        }
    }
}

impl FromResidual for CustomActionResult {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        match residual.0.into() {
            1602 => CustomActionResult::Cancel,
            _ => CustomActionResult::Fail,
        }
    }
}

impl<E> FromResidual<Result<Infallible, E>> for CustomActionResult
where
    E: std::error::Error,
{
    fn from_residual(_: Result<Infallible, E>) -> Self {
        CustomActionResult::Fail
    }
}

#[allow(non_camel_case_types)]
pub type c_char = i8;

#[no_mangle]
pub extern "C" fn println_env(var: *const c_char) -> CustomActionResult {
    let var: &CStr = unsafe { CStr::from_ptr(var) };
    let var = var.to_str()?;

    let val = std::env::var(var)?;
    println!("{}: {}", var, val);

    CustomActionResult::Succeed
}
