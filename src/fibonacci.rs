use std::ops::Mul;
use num_bigint::{BigUint, ToBigUint};

#[derive(Clone)]
struct Matrix {
    a: BigUint,
    b: BigUint,
    c: BigUint,
    d: BigUint,
}

pub fn fibonacci(n: BigUint) -> BigUint {
    let base = Matrix::new(1, 1, 1, 0);

    if n < BigUint::try_from(2).unwrap() { return n }

    base.quick_pow(n - 1.to_biguint().unwrap())
        .unwrap()
        .a
}

impl Mul<Matrix> for Matrix{
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix {
            a: self.a.clone() * rhs.a.clone() + self.b.clone() * rhs.c.clone(),
            b: self.a.clone() * rhs.b.clone() + self.b.clone() * rhs.d.clone(),
            c: self.c.clone() * rhs.a.clone() + self.d.clone() * rhs.c.clone(),
            d: self.c.clone() * rhs.b.clone() + self.d.clone() * rhs.d.clone(),
        }
    }
}

impl Matrix {
    fn new(a:u64, b:u64, c:u64, d:u64) -> Matrix {
        Matrix {
            a: BigUint::try_from(a).unwrap(),
            b: BigUint::try_from(b).unwrap(),
            c: BigUint::try_from(c).unwrap(),
            d: BigUint::try_from(d).unwrap(),
        }
    }

    fn quick_pow(self, power: BigUint) -> Option<Matrix> {
        if 0.to_biguint().unwrap() == power { return None; }
        if 1.to_biguint().unwrap() == power { return Some(self); }

        let mut result = self.clone();

        power.to_str_radix(2)
            .chars()
            .skip(1)// The first digit in a binary integer is always 1, and represents the first base in the algorithm
            .for_each(|digit| {
                result = result.clone() * result.clone();
                if '1' == digit { result = result.clone() * self.clone() };
            });

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use num_bigint::ToBigUint;
    use super::*;

    #[test]
    fn test_small() {
        assert_eq!(0.to_biguint().unwrap(), fibonacci(0.to_biguint().unwrap()));
        assert_eq!(1.to_biguint().unwrap(), fibonacci(1.to_biguint().unwrap()));
        assert_eq!(1.to_biguint().unwrap(), fibonacci(2.to_biguint().unwrap()));
        assert_eq!(2.to_biguint().unwrap(), fibonacci(3.to_biguint().unwrap()));
    }

    #[test]
    fn test_big() {
        assert_eq!(BigUint::from_str("12200160415121876738").unwrap(), fibonacci(93.to_biguint().unwrap()));
    }

    #[test]
    fn test_huge() {
        fibonacci(2000.to_biguint().unwrap());
    }
}