use crate::error::StatsError;
use std::fmt::Debug;

// The tolerance required between expected and actual floating point values in all of the tests.
const TOL: f64 = 1e-14;

// Return the type of the given value as a &'static str, useful for debugging the generic types
// being checked in the tests.
fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

// Panic, showing the values that differ and their types.
fn panic_with_types<T>(act: T, exp: T, line: u32)
where
    T: Debug + Copy,
{
    panic!(
        "found {:?} (type: {}), but expected {:?} (type: {}) for line {}",
        act,
        type_of(act),
        exp,
        type_of(exp),
        line
    );
}

// The Checker trait is implemented on the types we want to check. So `self` will the the actual
// value of a actual/expected comparison. Also parameterize the trait on T, which will be the
// expected value.
pub trait Checker<T> {
    fn assert(self, exp: T, line: u32);
}

impl Checker<f64> for f64 {
    fn assert(self, exp: f64, line: u32) {
        // Can use abs_diff_eq!() or relative_eq!().
        if !approx::relative_eq!(self, exp, epsilon = TOL) {
            panic_with_types(self, exp, line);
        }
    }
}

impl Checker<u32> for u32 {
    fn assert(self, exp: u32, line: u32) {
        if self != exp {
            panic_with_types(self, exp, line);
        }
    }
}

impl Checker<Result<f64, StatsError>> for Result<f64, StatsError> {
    fn assert(self, exp: Result<f64, StatsError>, line: u32) {
        match (self, exp) {
            (Err(err_act), Err(err_exp)) => {
                if err_act != err_exp {
                    panic_with_types(err_act, err_exp, line);
                }
            }
            (Ok(a), Ok(e)) => Checker::assert(a, e, line),
            _ => {
                panic_with_types(self, exp, line);
            }
        }
    }
}

// Define a macro and export it for use as the main testing function. Using a macro provides 1) a
// short name instead of `Checker::assert` and 2), the line number is defined at the call site, so
// is correct.
#[macro_export]
macro_rules! chk {
    ($e:expr, $value:expr) => {
        crate::tests::check::Checker::assert($e, $value, line!())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_fns() {
        Checker::assert(0u32, 0u32, line!());
        Checker::assert(1u32, 1u32, line!());
        Checker::assert(0.0, 0.0, line!());
        Checker::assert(1.0, 1.0, line!());
        Checker::assert(
            Err(StatsError::NotEnoughData),
            Err(StatsError::NotEnoughData),
            line!(),
        );
        Checker::assert(Ok(0.0), Ok(0.0), line!());
        Checker::assert(Ok(1.0), Ok(1.0), line!());
    }

    #[test]
    fn test_checks_macro() {
        chk!(0u32, 0u32);
        chk!(1u32, 1u32);
        chk!(0.0, 0.0);
        chk!(1.0, 1.0);
        chk!(
            Err(StatsError::NotEnoughData),
            Err(StatsError::NotEnoughData)
        );
        chk!(Ok(0.0), Ok(0.0));
        chk!(Ok(1.0), Ok(1.0));
    }

    #[test]
    #[should_panic]
    fn test_check_panic0() {
        chk!(0u32, 1u32);
    }

    #[test]
    #[should_panic]
    fn test_check_panic1() {
        chk!(0.0, 1.0);
    }

    #[test]
    #[should_panic]
    fn test_check_panic2() {
        chk!(Ok(0.0), Err(StatsError::NotEnoughData));
    }

    #[test]
    #[should_panic]
    fn test_check_panic3() {
        chk!(Err(StatsError::NotEnoughData), Ok(7.0));
    }

    #[test]
    #[should_panic]
    fn test_check_panic4() {
        chk!(Ok(6.0), Ok(8.0));
    }
}
