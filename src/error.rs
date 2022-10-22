use std::num::{ParseFloatError, ParseIntError};

use num_bigint::ParseBigIntError;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ParseBigDecimalError {
    #[error(transparent)]
    ParseDecimal(#[from] ParseFloatError),
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error(transparent)]
    ParseBigInt(#[from] ParseBigIntError),

    #[error("failed to parse empty string")]
    Empty,

    #[error("{0}")]
    Other(String),
}

// impl Error for ParseBigDecimalError {
//     fn description(&self) -> &str {
//         "failed to parse bigint/biguint"
//     }
// }
