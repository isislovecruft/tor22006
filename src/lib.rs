#![cfg_attr(feature = "bench", feature(test))]

extern crate core;

#[cfg(all(test, feature = "bench"))]
extern crate test;

#[cfg(test)]
extern crate rand;

extern crate curve25519_dalek;

use curve25519_dalek::constants;

use curve25519_dalek::curve::CompressedEdwardsY;
use curve25519_dalek::curve::ExtendedPoint;
use curve25519_dalek::curve::Identity;
use curve25519_dalek::curve::IsIdentity;

use curve25519_dalek::decaf::CompressedDecaf;
use curve25519_dalek::decaf::DecafPoint;


// The public key for an ed25519 scheme is a compressed edwards point (the
// Y-coordinate and the sign of X).

pub fn mult_by_cofactor_and_validate(key: &CompressedEdwardsY) -> Option<ExtendedPoint> {
    // decompression of Y in any sane curve25519 library should check validation
    // by computing:
    //
    //    Z ← fe(1)
    //    u ← Y² - Z
    //    v ← dY² + Z
    //    check ← sqrt(u/v)
    //
    // If `v` is nonzero and `check` is okay (meaning that `u/v` is square),
    // then the point is valid.
    let p: Option<ExtendedPoint> = key.decompress();
    let q: ExtendedPoint;
    
    match p.is_some() {
        true  => q = &p.unwrap() * &constants::l,
        false => return None, // the point was invalid
    }

    // We need to check that p*l is the identity (the identity point
    // is X:Y:Z:T == 0:1:1:0 since this shows there is no torsion
    // component)
    match q.is_identity() {
        true  => return Some(q),
        false => return None, // point * l was the identity
    }
}

// Decaf decompression ensures both that the point is a valid point on
// the curve and that it is within a prime-order group.
pub fn decaf_decompress(key: &CompressedDecaf) -> Option<DecafPoint> {
    let p: Option<DecafPoint>;

    // The constant-time equality check for a decaf point is only
    // defined for the compressed version, and works by byte comparison.
    match *key == CompressedDecaf::identity() {
        true  => return None, // the point was the identity
        false => p = key.decompress(),
    }

    match p.is_some() {
        true  => return p,
        false => return None, // invalid decaf point
    }
}

#[cfg(all(test, not(feature = "bench")))]
mod test {
    use super::*;

    use rand::OsRng;

    use curve25519_dalek::constants;
    use curve25519_dalek::scalar::Scalar;

    #[test]
    fn current_design() {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let a: Scalar = Scalar::random(&mut csprng);
        let p: ExtendedPoint = &a * &constants::ED25519_BASEPOINT;
        let key: CompressedEdwardsY = p.compress_edwards();

        let check = mult_by_cofactor_and_validate(&key);
        assert!(check.is_some());
    }

    #[test]
    fn with_decaf_instead() {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let p: DecafPoint = DecafPoint::random(&mut csprng);
        let key: CompressedDecaf = p.compress();

        let check = decaf_decompress(&key);
        assert!(check.is_some());
    }
}

#[cfg(all(test, feature = "bench"))]
mod bench {
    use super::*;

    use test::Bencher;
    use rand::OsRng;

    use curve25519_dalek::scalar::Scalar;

    #[bench]
    fn current_design(b: &mut Bencher) {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let a: Scalar = Scalar::random(&mut csprng);
        let p: ExtendedPoint = &a * &constants::ED25519_BASEPOINT;
        let key: CompressedEdwardsY = p.compress_edwards();

        b.iter(| | mult_by_cofactor_and_validate(&key) )
    }

    #[bench]
    fn with_decaf_instead(b: &mut Bencher) {
        let mut csprng: OsRng = OsRng::new().unwrap();
        let p: DecafPoint = DecafPoint::random(&mut csprng);
        let key: CompressedDecaf = p.compress();

        b.iter(| | decaf_decompress(&key) )
    }
}
