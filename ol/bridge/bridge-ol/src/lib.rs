#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod config;
pub mod contract;
pub mod sign_tx;
pub mod submit_tx;
pub mod save_tx;
