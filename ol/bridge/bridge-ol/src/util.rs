use std::convert::TryInto;
pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> Result<[T; N], String> {
    v.try_into()
        .map_err(|v: Vec<T>| format!("Expected a Vec of length {} but it was {}", N, v.len()))
}
