use strum_macros::EnumIter;

#[cfg(test)]
pub(crate) mod mock;

pub(crate) mod wrapper;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}

// impl From<u8> for VoteChoice {
//     fn from(value: u8) -> Self {
//         match value {
//             0 => Self::Yes,
//             1 => Self::No,
//             2 => Self::Abstain,
//             _ => panic!("Invalid vote choice"),
//         }
//     }
// }

// impl From<VoteChoice> for u8 {
//     fn from(value: VoteChoice) -> Self {
//         match value {
//             VoteChoice::Yes => 0,
//             VoteChoice::No => 1,
//             VoteChoice::Abstain => 2,
//         }
//     }
// }

impl From<&str> for VoteChoice {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "yes" => Self::Yes,
            "no" => Self::No,
            "abstain" => Self::Abstain,
            "y" => Self::Yes,
            "n" => Self::No,
            "a" => Self::Abstain,
            _ => panic!("Invalid vote choice"),
        }
    }
}
