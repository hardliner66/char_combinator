#[cfg(not(feature="bigint"))]
mod non_repeating_strings_u128;
#[cfg(not(feature="bigint"))]
pub use non_repeating_strings_u128::{DEFAULT_RANGE, NonRepeatingStrings};

#[cfg(feature="bigint")]
mod non_repeating_strings_bigint;
#[cfg(feature="bigint")]
pub use non_repeating_strings_bigint::{DEFAULT_RANGE, NonRepeatingStrings};

#[cfg(test)]
mod tests {
    use crate::NonRepeatingStrings;

    #[test]
    fn it_works() {
        assert_eq!(
            NonRepeatingStrings::default().skip(1024).nth(0).unwrap(),
            "sK"
        );
    }
}
