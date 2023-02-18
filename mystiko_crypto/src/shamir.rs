use crate::constants::FIELD_SIZE;
use crate::error::SecretShareError;
use crate::utils::{calc_mod, random_big_int};
use num_bigint::BigInt;
use num_traits::identities::Zero;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: BigInt,
    y: BigInt,
}

pub struct SecretShare {
    pub num_of_shares: u32,
    pub threshold: u32,
    pub shares: Vec<Point>,
    pub coefficients: Vec<BigInt>,
}

pub fn recover(shares: Vec<Point>, in_prime: Option<BigInt>) -> BigInt {
    let prime = match in_prime {
        Some(a) => a,
        _ => FIELD_SIZE.clone(),
    };

    lagrange_interpolate(BigInt::zero(), shares, prime)
}

pub fn split(
    secret: BigInt,
    num_of_shares: u32,
    threshold: u32,
    in_prime: Option<BigInt>,
) -> Result<SecretShare, SecretShareError> {
    let prime = match in_prime {
        Some(a) => a,
        _ => FIELD_SIZE.clone(),
    };

    if num_of_shares == 0 || num_of_shares > 2u32.pow(13) {
        return Err(SecretShareError::SharesOutOfBounds);
    }

    if threshold == 0 || threshold > num_of_shares {
        return Err(SecretShareError::ThresholdOutOfBounds);
    }

    let mut coefficients: Vec<BigInt> = vec![secret];
    for _ in 1..threshold {
        coefficients.push(random_big_int(32, &prime));
    }

    let mut shares: Vec<Point> = vec![];
    for i in 0..num_of_shares {
        let x = BigInt::from(i + 1);
        let y = eval_poly(&coefficients, &x, &prime);
        shares.push(Point { x, y });
    }

    Ok(SecretShare {
        num_of_shares,
        threshold,
        coefficients,
        shares,
    })
}

fn batch_mul(values: &[BigInt]) -> BigInt {
    let mut accum = BigInt::from(1u32);
    values.iter().for_each(|v| accum *= v);
    accum
}

fn eval_poly(coefficients: &[BigInt], x: &BigInt, prime: &BigInt) -> BigInt {
    let mut accum = BigInt::zero();
    for cf in coefficients.iter().rev() {
        accum *= x;
        accum += cf;
        accum = calc_mod(accum, prime);
    }
    accum
}

fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt) {
    let mut x = BigInt::zero();
    let mut last_x = BigInt::from(1u32);
    let mut y = BigInt::from(1u32);
    let mut last_y = BigInt::zero();

    let mut a_val = a;
    let mut b_val = b;

    while !b_val.is_zero() {
        let quote = a_val.clone() / b_val.clone();
        let temp_b = calc_mod(a_val.clone(), &b_val);
        a_val = b_val.clone();
        b_val = temp_b;
        let temp_x = last_x - (quote.clone() * x.clone());
        let temp_y = last_y - (quote * y.clone());
        last_x = x.clone();
        last_y = y.clone();
        x = temp_x;
        y = temp_y;
    }

    (last_x, last_y)
}

fn div_mod(num: BigInt, den: BigInt, prime: BigInt) -> BigInt {
    let (x, _) = extended_gcd(den, prime);
    x * num
}

fn sum(values: Vec<BigInt>) -> BigInt {
    values.iter().fold(BigInt::zero(), |acc, x| acc + x)
}

fn lagrange_interpolate(x: BigInt, points: Vec<Point>, prime: BigInt) -> BigInt {
    let k = points.len();
    let mut hashset = HashSet::new();
    assert_eq!(
        k,
        points
            .iter()
            .filter(|p| hashset.insert(p.x.clone()))
            .count()
    );

    let mut nums: Vec<BigInt> = vec![];
    let mut dens: Vec<BigInt> = vec![];

    for i in 0..k {
        let mut num_values: Vec<BigInt> = vec![];
        let mut den_values: Vec<BigInt> = vec![];

        for j in 0..k {
            if j != i {
                num_values.push(x.clone() - points[j].clone().x);
                den_values.push(points[i].clone().x - points[j].clone().x)
            }
        }

        nums.push(batch_mul(&num_values));
        dens.push(batch_mul(&den_values));
    }

    let den = batch_mul(&dens);
    let mut num_values: Vec<BigInt> = vec![];
    for i in 0..k {
        let num = calc_mod(nums[i].clone() * den.clone() * points[i].clone().y, &prime);
        num_values.push(div_mod(num, dens[i].clone(), prime.clone()));
    }
    let num = sum(num_values);
    calc_mod(div_mod(num, den, prime.clone()) + prime.clone(), &prime)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;

    #[test]
    fn test_secret_sharing() {
        let secret = random_big_int(32, &FIELD_SIZE);
        let result = split(secret.clone(), 0, 17, None);
        assert_eq!(result.err().unwrap(), SecretShareError::SharesOutOfBounds);

        let result = split(secret.clone(), 5, 7, None);
        assert_eq!(
            result.err().unwrap(),
            SecretShareError::ThresholdOutOfBounds
        );
    }

    #[test]
    fn test_secret_sharing1() {
        let secret = random_big_int(32, &FIELD_SIZE);
        let ss = split(secret.clone(), 30, 17, None).unwrap();
        let mut shares = ss.shares.clone();
        shares.shuffle(&mut rand::thread_rng());
        let es = shares[0..17].to_vec();
        let recovered_secret = recover(es, None);
        assert_eq!(secret, recovered_secret);
    }

    #[test]
    fn test_secret_sharing2() {
        let mut secret = random_big_int(32, &FIELD_SIZE);
        let field: BigInt = BigInt::parse_bytes(b"58995116542422524421248775517049", 10).unwrap();
        secret = calc_mod(secret, &field);

        let ss = split(secret.clone(), 5, 3, Some(field.clone())).unwrap();
        let mut shares = ss.shares.clone();
        shares.shuffle(&mut rand::thread_rng());
        let es = shares[0..3].to_vec();
        let recovered_secret = recover(es, Some(field.clone()));
        assert_eq!(secret, recovered_secret);
    }
}
