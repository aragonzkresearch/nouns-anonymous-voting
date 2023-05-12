use ark_std::rand::Rng;

/// Mock trait is used to generate mock data for testing
pub trait Mock {
    fn mock<R: Rng>(rng: &mut R) -> Self;
}