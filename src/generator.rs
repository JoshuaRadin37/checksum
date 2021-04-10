pub trait Generator {
    type Output: Eq;

    fn generate_checksum(self) -> Self::Output;
}