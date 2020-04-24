pub mod digit;
pub use digit::*;

/// if true:
///     return "Yes"
/// if false:
///     return "No"
pub fn yes_no(judge: bool) -> &'static str {
    if judge {
        "Yes"
    } else {
        "No"
    }
}
