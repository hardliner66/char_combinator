#[cfg(feature = "bigint")]
use num_bigint::BigUint;

#[cfg(feature = "bigint")]
type Counter = BigUint;

#[cfg(not(feature = "bigint"))]
type Counter = u128;

use num_traits::cast::ToPrimitive;
use num_traits::{One, Zero};

/// Default range from a to Z
pub const DEFAULT_RANGE: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Creates all combinations from the given range
///
/// # Examples
///
/// ```
/// use char_combinator::{CharCombinator};
///
/// fn main() {
///     let strings: Vec<String> = CharCombinator::default().skip(123).take(4).collect();
///     assert_eq!(dbg!(strings), vec!["bt", "bu", "bv", "bw"]);
/// }
/// ```
#[derive(Clone)]
pub struct CharCombinator<'a> {
    current: Counter,
    range: &'a [char],
}

impl<'a> CharCombinator<'a> {
    /// Creates a new iterator with a specified range
    pub fn new(range: &'a [char]) -> CharCombinator<'a> {
        CharCombinator {
            current: Zero::zero(),
            range,
        }
    }

    /// Creates a new iterator with a specified range, starting at the given count
    pub fn new_from(start: Counter, range: &'a [char]) -> CharCombinator<'a> {
        CharCombinator {
            current: start,
            range,
        }
    }

    /// Returns the current count of the iterator.
    pub fn current(&self) -> &Counter {
        &self.current
    }
}

impl Default for CharCombinator<'static> {
    fn default() -> Self {
        CharCombinator::new(&DEFAULT_RANGE)
    }
}

#[inline(always)]
fn to_letters(range: &[char], u: &Counter) -> String {
    let mut result = String::new();

    let mut quotient = u.clone();

    let range_len: Counter = (range.len() as u64).into();

    // until we have a 0 quotient
    while quotient != Zero::zero() {
        // compensate for 0 based array
        let decremented = quotient - Counter::from(1u64);

        // divide by 52
        quotient = &decremented / &range_len;

        // get remainder
        let remainder = &decremented % &range_len;

        // prepend the letter at index of remainder
        if cfg!(target_pointer_width = "64") {
            result = range[remainder.to_u64().unwrap() as usize].to_string() + &result;
        } else {
            result = range[remainder.to_u32().unwrap() as usize].to_string() + &result;
        }
    }

    return result;
}

impl<'a> Iterator for CharCombinator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let one: Counter = One::one();
        self.current += one;

        Some(to_letters(&self.range, &self.current))
    }
}

#[cfg(test)]
mod tests {
    use crate::CharCombinator;

    #[test]
    fn default() {
        assert_eq!(CharCombinator::default().skip(1024).nth(0).unwrap(), "sK");
    }

    #[test]
    fn with_range() {
        assert_eq!(
            CharCombinator::new(&vec!['a', 'b', 'c'])
                .skip(1024)
                .nth(0)
                .unwrap(),
            "ccabbb"
        );
    }

    #[test]
    fn start_at() {
        assert_eq!(
            CharCombinator::new_from(1024u64.into(), &vec!['a', 'b', 'c'])
                .nth(0)
                .unwrap(),
            "ccabbb"
        );
    }
}
