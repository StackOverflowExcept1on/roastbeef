use roast_core::tests;
use roast_p256::{error::Error, frost::rand_core::OsRng};

#[test]
fn test_dkg_basic() -> Result<(), Error> {
    let mut rng = OsRng;
    tests::test_dkg_basic(2, 3, &mut rng)?;
    Ok(())
}

#[test]
fn test_basic() -> Result<(), Error> {
    let mut rng = OsRng;
    tests::test_basic(2, 3, &mut rng)?;
    tests::test_basic(67, 100, &mut rng)?;
    Ok(())
}

#[test]
fn test_malicious() -> Result<(), Error> {
    let mut rng = OsRng;
    tests::test_malicious(2, 3, 1, &mut rng)?;
    tests::test_malicious(67, 100, 33, &mut rng)?;
    Ok(())
}
