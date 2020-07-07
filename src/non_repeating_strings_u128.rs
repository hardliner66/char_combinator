use num_traits::cast::ToPrimitive;
use num_traits::{One, Zero};

pub const DEFAULT_RANGE: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[derive(Clone)]
pub struct NonRepeatingStrings<'a> {
    current: u128,
    range: &'a [char],
}

impl<'a> NonRepeatingStrings<'a> {
    pub fn new(range: &'a [char]) -> NonRepeatingStrings<'a> {
        NonRepeatingStrings {
            current: Zero::zero(),
            range,
        }
    }

    pub fn new_from(start: u128, range: &'a [char]) -> NonRepeatingStrings<'a> {
        NonRepeatingStrings {
            current: start,
            range,
        }
    }

    pub fn current(&self) -> &u128 {
        &self.current
    }
}

impl Default for NonRepeatingStrings<'static> {
    fn default() -> Self {
        NonRepeatingStrings::new(&DEFAULT_RANGE)
    }
}

#[inline(always)]
fn to_letters(range: &[char], u: u128) -> String {
    let mut result = String::new();

    let mut quotient = u;

    let range_len: u128 = (range.len() as u64).into();

    // until we have a 0 quotient
    while quotient != Zero::zero() {
        // compensate for 0 based array
        let decremented = quotient - u128::from(1u64);

        // divide by 52
        quotient = decremented / range_len;

        // get remainder
        let remainder = decremented % range_len;

		// prepend the letter at index of remainder
		if cfg!(target_pointer_width = "64") {
			result = range[remainder.to_u64().unwrap() as usize].to_string() + &result;
		} else {
			result = range[remainder.to_u32().unwrap() as usize].to_string() + &result;
		}
    }

    return result;
}

impl<'a> Iterator for NonRepeatingStrings<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let one: u128 = One::one();
        self.current += one;

        Some(to_letters(&self.range, self.current))
    }
}
