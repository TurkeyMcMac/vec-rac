#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub const VERSION: u32 = 1;
    pub const RAND_MAX: u64 = MODULUS - 1;

    pub fn with_seed(seed: u64) -> Rng {
        Rng {
            state: seed % MODULUS,
        }
    }

    pub fn forward(&mut self) -> u64 {
        let here = self.state;
        self.state = (MULTIPLIER * self.state + INCREMENT) % MODULUS;
        here
    }

    pub fn backward(&mut self) -> u64 {
        // From https://stackoverflow.com/a/29585823/11815766
        self.state = INVERSE_MULTIPLIER * (self.state - INCREMENT) % MODULUS;
        self.state
    }
}

// Taken from https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
// This is apparently used by C implementations.
const MULTIPLIER: u64 = 1103515245;
const INCREMENT: u64 = 12345;
const MODULUS: u64 = 2147483648;
const INVERSE_MULTIPLIER: u64 = 1857678181;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rng_goes_both_ways() {
        let mut rng = Rng::with_seed(12304);
        let forward = std::iter::repeat_with(|| rng.forward())
            .take(5)
            .collect::<Vec<_>>();
        let mut backward = std::iter::repeat_with(|| rng.backward())
            .take(5)
            .collect::<Vec<_>>();
        backward.reverse();
        assert_eq!(forward, backward);
    }

    #[test]
    fn inverse_multiplier_is_correct() {
        assert!(MULTIPLIER * INVERSE_MULTIPLIER % MODULUS == 1);
    }

    #[test]
    fn no_overflow() {
        let mut rng = Rng::with_seed(17700000001);
        rng.forward();
    }
}
