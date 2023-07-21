use crate::constants::FIELD_SIZE;
use crate::error::SecretShareError;
use crate::num_traits::One;
use crate::utils::{mod_floor, random_biguint};
use anyhow::Result;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_integer::Integer;
use num_traits::identities::Zero;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: BigUint,
    pub y: BigUint,
}

#[derive(Debug, Clone)]
pub struct SecretShare {
    pub num_of_shares: u32,
    pub threshold: u32,
    pub shares: Vec<Point>,
    pub coefficients: Vec<BigUint>,
}

pub fn recover(shares: Vec<Point>, in_prime: Option<BigUint>) -> BigUint {
    let prime = match in_prime {
        Some(a) => a,
        _ => FIELD_SIZE.clone(),
    };

    lagrange_interpolate(&BigUint::zero(), &shares, &prime)
}

pub fn split(
    secret: BigUint,
    num_of_shares: u32,
    threshold: u32,
    in_prime: Option<BigUint>,
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

    let mut coefficients: Vec<BigUint> = vec![secret];
    for _ in 1..threshold {
        coefficients.push(random_biguint(32, &prime));
    }

    let mut shares: Vec<Point> = vec![];
    for i in 0..num_of_shares {
        let x = BigUint::from(i + 1);
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
    values.iter().product()
}

fn eval_poly(coefficients: &[BigUint], x: &BigUint, prime: &BigUint) -> BigUint {
    let mut accum = BigUint::zero();
    for cf in coefficients.iter().rev() {
        accum *= x;
        accum += cf;
        accum = mod_floor(&accum, prime);
    }
    accum
}

fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
    let mut x = BigInt::zero();
    let mut last_x = BigInt::one();
    let mut y = BigInt::one();
    let mut last_y = BigInt::zero();

    let mut a_val = a.clone();
    let mut b_val = b.clone();

    while !b_val.is_zero() {
        let quote = a_val.clone() / b_val.clone();
        let temp_b = bigint_mod_floor(&a_val, &b_val);
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

fn bigint_mod_floor(a_number: &BigInt, prime: &BigInt) -> BigInt {
    a_number.mod_floor(prime)
}

fn div_mod(num: &BigInt, den: &BigInt, prime: &BigInt) -> BigInt {
    let (x, _) = extended_gcd(den, prime);
    x * num
}

fn sum(values: &[BigInt]) -> BigInt {
    values.iter().sum()
}

fn lagrange_interpolate(x: &BigUint, points: &[Point], prime: &BigUint) -> BigUint {
    let k = points.len();
    let mut hashset = HashSet::new();
    assert_eq!(k, points.iter().filter(|p| hashset.insert(p.x.clone())).count());

    let mut nums: Vec<BigInt> = vec![];
    let mut dens: Vec<BigInt> = vec![];

    for i in 0..k {
        let mut num_values: Vec<BigInt> = vec![];
        let mut den_values: Vec<BigInt> = vec![];

        for j in 0..k {
            if j != i {
                num_values.push(x.clone().to_bigint().unwrap() - points[j].clone().x.to_bigint().unwrap());
                den_values.push(points[i].clone().x.to_bigint().unwrap() - points[j].clone().x.to_bigint().unwrap())
            }
        }

        nums.push(batch_mul(&num_values));
        dens.push(batch_mul(&den_values));
    }

    let prime_bigint: BigInt = prime.clone().into();
    let den = batch_mul(&dens);
    let mut num_values: Vec<BigInt> = vec![];
    for i in 0..k {
        let num = bigint_mod_floor(
            &(nums[i].clone() * den.clone() * points[i].y.clone().to_bigint().unwrap()),
            &prime_bigint,
        );
        num_values.push(div_mod(&num, &dens[i], &prime_bigint));
    }
    let num = sum(num_values.as_slice());
    bigint_mod_floor(&(div_mod(&num, &den, &prime_bigint) + &prime_bigint), &prime_bigint)
        .to_biguint()
        .unwrap()
}
