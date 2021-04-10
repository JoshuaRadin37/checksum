pub trait Checker{
    type Input: Eq;

    /// Verifies a checksum
    ///
    /// A checker should be stateless, so verifying the checksum shouldn't need a mutable reference
    fn verify_checksum(&self, input: Self::Input) -> bool;
}