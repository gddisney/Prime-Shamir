use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

/// Generates a large prime number with the specified bit size.
pub fn generate_large_prime(bits: usize) -> BigUint {
    let mut rng = ChaCha20Rng::from_entropy();
    loop {
        let candidate = rng.gen_biguint(bits as u64) | BigUint::one(); // Ensure it's odd
        if is_probably_prime(&candidate, 10) {
            return candidate;
        }
    }
}

/// Rabin-Miller primality test for probabilistic prime checking.
fn is_probably_prime(n: &BigUint, k: usize) -> bool {
    if *n <= BigUint::from(1u64) {
        return false;
    }
    if *n == BigUint::from(2u64) {
        return true;
    }
    if n % 2u64 == BigUint::zero() {
        return false;
    }

    let mut rng = ChaCha20Rng::from_entropy();
    let one = BigUint::one();
    let two = &one + &one;
    let n_minus_one = n - &one;

    // Write n-1 as 2^s * d
    let mut d = n_minus_one.clone();
    let mut s = 0;
    while &d % &two == BigUint::zero() {
        d /= &two;
        s += 1;
    }

    'outer: for _ in 0..k {
        let a = rng.gen_biguint_range(&two, n); // Random a in [2, n-1]
        let mut x = a.modpow(&d, n);

        if x == one || x == n_minus_one {
            continue;
        }

        for _ in 0..(s - 1) {
            x = x.modpow(&two, n);
            if x == n_minus_one {
                continue 'outer;
            }
        }

        return false; // Composite
    }

    true // Probably prime
}

/// Generates Shamir secret shares without enforcing primality on `y` values.
fn shamir_split_shares(
    secret: &BigUint,
    threshold: usize,
    shares: usize,
    modulus: &BigUint,
) -> Vec<(usize, BigUint)> {
    assert!(threshold > 1, "Threshold must be at least 2");
    assert!(shares >= threshold, "Number of shares must be >= threshold");

    let mut rng = ChaCha20Rng::from_entropy();
    let mut coefficients = Vec::with_capacity(threshold);

    // Generate random coefficients for the polynomial (degree = threshold - 1)
    coefficients.push(secret.clone()); // The secret is the constant term
    for _ in 1..threshold {
        coefficients.push(rng.gen_biguint_below(modulus));
    }

    // Generate the shares
    let mut result = Vec::with_capacity(shares);
    for x in 1..=shares {
        let x_biguint = BigUint::from(x as u64);
        let mut y = BigUint::zero();
        for (i, coeff) in coefficients.iter().enumerate() {
            let term = coeff * x_biguint.modpow(&BigUint::from(i as u64), modulus);
            y = (y + term) % modulus;
        }
        result.push((x, y));
    }

    result
}

/// Reconstructs the secret using Lagrange Interpolation with precise modular arithmetic.
fn shamir_reconstruct(shares: &[(usize, BigUint)], modulus: &BigUint) -> BigUint {
    let mut reconstructed = BigUint::zero();

    for (i, (xi, yi)) in shares.iter().enumerate() {
        let mut numerator = BigUint::one();
        let mut denominator = BigUint::one();

        for (j, (xj, _)) in shares.iter().enumerate() {
            if i != j {
                let xj_big = BigUint::from(*xj as u64);
                let xi_big = BigUint::from(*xi as u64);

                // Compute (xj - xi) % modulus and handle negative values
                let diff = (xj_big.clone() + modulus - xi_big.clone()) % modulus;

                numerator = (numerator * xj_big) % modulus;
                denominator = (denominator * diff) % modulus;
            }
        }

        // Compute modular inverse of the denominator using Fermat's Little Theorem
        let denominator_inv = denominator.modpow(&(modulus - BigUint::from(2u64)), modulus);

        // Compute Lagrange coefficient
        let lagrange_coeff = (numerator * denominator_inv) % modulus;

        // Add the current term to the reconstructed value
        let term = (lagrange_coeff * yi) % modulus;
        reconstructed = (reconstructed + term) % modulus;
    }

    reconstructed
}

/// Verifies whether each share is prime (for informational purposes).
fn verify_share_primality(shares: &[(usize, BigUint)]) {
    for (x, y) in shares {
        if is_probably_prime(y, 10) {
            println!("Share at x = {} is prime.", x);
        } else {
            println!("Share at x = {} is NOT prime.", x);
        }
    }
}

fn main() {
    let secret_bits = 512;
    let secret = generate_large_prime(secret_bits);

    // Use a modulus significantly larger than the secret to avoid wrap-around
    let modulus_bits = secret_bits * 2;
    let modulus = generate_large_prime(modulus_bits);

    let threshold = 6; // Minimum shares needed to reconstruct
    let shares_count = 8; // Total shares to generate

    let shares = shamir_split_shares(&secret, threshold, shares_count, &modulus);

    println!("Original Secret (Prime): {}", secret);
    println!("Shares:");
    for (x, y) in &shares {
        println!("x: {}, y: {}", x, y);
    }

    // Optionally, verify primality of shares (expected to be NOT prime)
    verify_share_primality(&shares);

    // Select the first `threshold` shares for reconstruction
    let selected_shares = &shares[..threshold];
    let reconstructed_secret = shamir_reconstruct(selected_shares, &modulus);

    println!("Reconstructed Secret: {}", reconstructed_secret);

    assert_eq!(
        secret, reconstructed_secret,
        "Reconstructed secret does not match the original secret!"
    );

    println!("Reconstruction successful. The secret matches exactly.");
}

