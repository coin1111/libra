use std::convert::TryInto;
use anyhow::{Error, anyhow};
pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> Result<[T; N], Error> {
    v.try_into()
        .map_err(|v: Vec<T>| anyhow!("Expected a Vec of length {} but it was {}", N, v.len()))
}
