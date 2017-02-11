const MANTISSA_BITS: i32 = 52;
const MANTISSA_MASK: u64 = (1u64 << MANTISSA_BITS) - 1;

fn ldexp(x: u64, exp: i32) -> f64 {
    (x as f64) * f64::exp2(exp as f64)
}

// https://en.wikipedia.org/wiki/Lehmer_random_number_generator
fn lcg_parkmiller(seed: u32) -> u32 {
    (((seed as u64) * 48_271_u64) % 2_147_483_647_u64) as u32
}

// http://xorshift.di.unimi.it/splitmix64.c
fn splitmix64(seed: u64) -> u64 {
    let mut z = seed + 0x9E3779B97F4A7C15_u64;
    z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9_u64;
    z = (z ^ (z >> 27)) * 0x94D049BB133111EB_u64;
    return z ^ (z >> 31);
}

fn u64_from_bytes (bytes: &[u8]) -> u64 {
    ((bytes[7] as u64) << 56) + ((bytes[6] as u64) << 48) +
    ((bytes[5] as u64) << 40) + ((bytes[4] as u64) << 32) +
    ((bytes[3] as u64) << 24) + ((bytes[2] as u64) << 16) +
    ((bytes[1] as u64) <<  8) + ((bytes[0] as u64) <<  0)
}

/// # Examples
///
/// Construct a RNG from an integer.
///
/// ```
/// extern crate xorshift128plus;
///
/// use xorshift128plus::XorShift128Plus;
///
/// # fn main() {
/// let mut rng = XorShift128Plus::from_u32(4293262078);
///
/// println!("First random float: {}", rng.next());
/// println!("Second random float: {}", rng.next());
/// # }
/// ```
///
/// You can also construct a RNG by supplying 16 bytes of raw data. Here we are using the [rand
/// crates `rand::random`](https://doc.rust-lang.org/rand/rand/fn.random.html) to generate 16 bytes
/// of random data to use as the initial seed. This data could also come from a file, a database or
/// anywhere else really.
///
/// ```
/// extern crate rand;
/// extern crate xorshift128plus;
///
/// use xorshift128plus::XorShift128Plus;
///
/// # fn main() {
/// let seed: [u8; 16] = rand::random();
/// let mut rng = XorShift128Plus::from_bytes(seed);
///
/// println!("First random float: {}", rng.next());
/// println!("Second random float: {}", rng.next());
/// # }
/// ```
pub struct XorShift128Plus (u64, u64);

impl XorShift128Plus {
    /// Constructs a new RNG with the seed specified as 16 bytes of raw data.
    pub fn from_bytes (seed: [u8; 16]) -> XorShift128Plus {
        XorShift128Plus (
            u64_from_bytes(&seed[0..8]),
            u64_from_bytes(&seed[8..16]),
        )
    }

    /// Constructs a new RNG with the seed specified as a unsigned 32bit integer. Note that
    /// this seeding is suboptimal since it will only contain 32 bits of entropy instead
    /// of 128 bits.
    pub fn from_u32 (seed: u32) -> XorShift128Plus {
        let raw0 = lcg_parkmiller(seed);
        let raw1 = lcg_parkmiller(raw0);
        let raw2 = lcg_parkmiller(raw1);
        let raw3 = lcg_parkmiller(raw2);

        XorShift128Plus (
            ((raw1 as u64) << 32) + (raw0 as u64),
            ((raw3 as u64) << 32) + (raw2 as u64),
        )
    }

    /// Constructs a new RNG with the seed specified as a unsigned 64bit integer. Note that
    /// this seeding is suboptimal since it will only contain 64 bits of entropy instead
    /// of 128 bits.
    pub fn from_u64 (seed: u64) -> XorShift128Plus {
        let raw0 = splitmix64(seed);
        let raw1 = splitmix64(raw0);

        XorShift128Plus (raw0, raw1)
    }

    /// Returns the next psuedo-random number between 0 (inclusivly) and 1 (exclusivly).
    pub fn next (&mut self) -> f64 {
        let mut x = self.0;
        let y = self.1;

        self.0 = y;

        x ^= x << 23;
        x ^= x >> 17;
        x ^= y;
        x ^= y >> 26;

        self.1 = x;

        ldexp(u64::wrapping_add(self.0, self.1) & MANTISSA_MASK, -MANTISSA_BITS)
    }
}

#[cfg(test)]
mod tests {
    use super::XorShift128Plus;

    #[test]
    fn it_should_seed_from_bytes() {
        let mut rng = XorShift128Plus::from_bytes([
            0x5d, 0x28, 0x94, 0x50, 0xc8, 0x88, 0xf9, 0x9b,
            0x5e, 0x5c, 0x1f, 0xd1, 0x35, 0x09, 0xe3, 0x9e,
        ]);

        assert_eq!(rng.next(), 0.35873106038177727);
        assert_eq!(rng.next(), 0.7433543130711686);
        assert_eq!(rng.next(), 0.6325316214071923);
        assert_eq!(rng.next(), 0.708663591569944);
        assert_eq!(rng.next(), 0.8974382234842848);
    }

    #[test]
    fn it_should_seed_from_u32() {
        let mut rng = XorShift128Plus::from_u32(4293262078);

        assert_eq!(rng.next(), 0.4335893835472515);
        assert_eq!(rng.next(), 0.6067907909036327);
        assert_eq!(rng.next(), 0.046905965279849804);
        assert_eq!(rng.next(), 0.480991995797152);
        assert_eq!(rng.next(), 0.6796126170804464);
    }
}
