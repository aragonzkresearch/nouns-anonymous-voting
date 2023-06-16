mod bn254_fr;
pub mod toml;

#[macro_export]
macro_rules! concat_vec {
    ($($arr:expr),*) => {
        {
            let mut result = Vec::new();
            $(
                result.extend($arr.iter());
            )*
            result
        }
    };
}

/// A wrapper type for defining serialisation traits for types that are not defined in this crate.
pub struct Wrapper<T>(pub T);
