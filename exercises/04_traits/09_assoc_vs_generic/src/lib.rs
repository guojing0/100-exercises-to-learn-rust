// Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

trait Power<Exp = Self> {
    type Output;

    fn power(self, exp: Exp) -> Self::Output;
}

impl Power<u16> for u16 {
    type Output = u16;

    fn power(self, exp: u16) -> Self::Output {
        let mut result = 1;

        for _ in 1..=exp {
           result *= self;
        }

        result
    }
}

impl Power<u32> for u32 {
    type Output = u32;

    fn power(self, exp: u32) -> Self::Output {
        let mut result = 1;

        for _ in 1..=exp {
           result *= self;
        }

        result
    }
}

impl Power<&u32> for u32 {
    type Output = u32;

    fn power(self, exp: &u32) -> Self::Output {
        let mut result = 1;

        for _ in 1..=*exp {
           result *= self;
        }

        result
    }
}


// pub trait Add<RHS = Self> {
//     type Output;
    
//     fn add(self, rhs: RHS) -> Self::Output;
// }


// impl From<u32> for WrappingU32 {
//     fn from(value: u32) -> Self {
//         WrappingU32 { inner: value }
//     }
// }

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u16 = 2_u16.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
