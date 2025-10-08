use crate::{bail, Context, Ok, Result};
use crate::trace;
use crate::function;

fn try_add_positive(a: &i32, b: &i32) -> Result<i32> {
    trace!(a, b, "try_add_positive");
    if *b < 0 {
        bail!("b < 0, unsupported ({}:{} {})", file!(), line!(), function!());
    }
    Ok(*a + *b)
}

fn try_add_negative(a: &i32, b: &i32) -> Result<i32> {
    trace!(a, b, "try_add_negative");
    if *b > 0 {
        bail!("b > 0, unsupported ({}:{} {})", file!(), line!(), function!());
    }
    Ok(*a + *b)
}

pub fn try_add(a: &i32, b: &i32) -> Result<i32> {
    trace!(a, b, "{}:{}", file!(),function!());
    let sum;
    if *a < 0 {
        sum = try_add_negative(a, b).context("try_add_negative failed")?;
    } else if *a > 0 {
        sum = try_add_positive(a, b).context("try_add_positive failed")?;
    } else {
        bail!("No supported add function for a = {} and b = {}", a, b);
    }
    Ok(sum)
}
