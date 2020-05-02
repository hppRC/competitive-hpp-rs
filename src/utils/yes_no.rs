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

/// if true:
///     return "YES"
/// if false:
///     return "NO"
#[allow(non_snake_case)]
pub fn YESNO(judge: bool) -> &'static str {
    if judge {
        "YES"
    } else {
        "NO"
    }
}
