pub mod digit;
pub use digit::*;

pub mod math;
pub use math::*;

/// if true:
///     return "Yes"
/// if false:
///     return "No"
#[allow(non_snake_case)]
pub fn YesNo(judge: bool) -> &'static str {
    if judge {
        "Yes"
    } else {
        "No"
    }
}
