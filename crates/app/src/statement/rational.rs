use minicbor::{Decode, Decoder, Encode, Encoder, decode, encode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rational(pub i64, pub u64);

impl Rational {
    pub fn new(num: i64, denom: u64) -> Self {
        if denom == 0 {
            panic!("Rational denom cannot be 0");
        };
        Self(num, denom)
    }

    pub fn zero() -> Self {
        Self(0, 1)
    }

    pub fn as_f64(&self) -> f64 {
        self.0 as f64 / self.1 as f64
    }

    pub fn from_f64(val: f64, max_den: u64) -> Self {
        if val.is_nan() || val.is_infinite() {
            return Self(0, 1);
        }

        let (is_neg, x) = (val < 0.0, val.abs());
        let mut a = x.floor() as i64;
        let mut r = x - a as f64;

        // Correct initialization: (p0, q0) = 1/0, (p1, q1) = a/1
        let (mut p0, mut q0, mut p1, mut q1) = (1, 0, a, 1);

        while r >= 1e-15 {
            r = 1.0 / r;
            a = r.floor() as i64;
            r -= a as f64;

            let next_den = q0 + a as u64 * q1;
            if next_den > max_den {
                let k = (max_den - q1) / q0;
                let (s_num, s_den) = (p0 + k as i64 * p1, q0 + k * q1);
                let use_semi =
                    (x - (s_num as f64 / s_den as f64)).abs() < (x - (p1 as f64 / q1 as f64)).abs();

                let res = if use_semi {
                    Self(s_num, s_den)
                } else {
                    Self(p1, q1)
                };
                return Self(if is_neg { -res.0 } else { res.0 }, res.1);
            }

            (p0, q0, p1, q1) = (p1, q1, p0 + a * p1, next_den);
        }

        Self(if is_neg { -p1 } else { p1 }, q1)
    }
}

impl<C> Encode<C> for Rational {
    fn encode<W: encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), encode::Error<W::Error>> {
        e.tag(minicbor::data::Tag::new(121))?
            .begin_array()?
            .encode(self.0)?
            .encode(self.1)?
            .end()?;
        Ok(())
    }
}

impl<'b, C> Decode<'b, C> for Rational {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, decode::Error> {
        if d.tag()? != minicbor::data::Tag::new(121) {
            return Err(decode::Error::message("Expected tag 121 for Rational"));
        }
        d.array()?;
        let num = d.decode()?;
        let denom = d.decode()?;
        if d.datatype()? == minicbor::data::Type::Break {
            d.skip()?;
        }
        Ok(Rational(num, denom))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PI: f64 = std::f64::consts::PI;

    #[test]
    fn test_pi_bounds() {
        // Classic approximations of Pi
        assert_eq!(Rational::from_f64(PI, 10), Rational(22, 7));
        assert_eq!(Rational::from_f64(PI, 400), Rational(355, 113));
    }

    #[test]
    fn test_exact_fractions() {
        // Exact fractions should resolve perfectly
        assert_eq!(Rational::from_f64(0.75, 10), Rational(3, 4));
        assert_eq!(Rational::from_f64(0.125, 10), Rational(1, 8));
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(Rational::from_f64(-0.75, 10), Rational(-3, 4));
        assert_eq!(Rational::from_f64(-PI, 10), Rational(-22, 7));
    }

    #[test]
    fn test_whole_numbers() {
        assert_eq!(Rational::from_f64(5.0, 10), Rational(5, 1));
        assert_eq!(Rational::from_f64(0.0, 10), Rational(0, 1));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(Rational::from_f64(f64::NAN, 10), Rational(0, 1));
        assert_eq!(Rational::from_f64(f64::INFINITY, 10), Rational(0, 1));
    }

    use minicbor::{decode, encode};

    #[test]
    fn test_rational_roundtrip() {
        let original = Rational::new(22, 7);
        let mut buffer = Vec::new();
        encode(original.clone(), &mut buffer).expect("Failed to encode Rational");
        let decoded: Rational = decode(&buffer).expect("Failed to decode Rational");

        assert_eq!(original, decoded);
    }
}
