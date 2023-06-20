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
