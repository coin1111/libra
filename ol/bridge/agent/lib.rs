// 0L

#![forbid(unsafe_code)]
// Increase recursion limit to allow for use of select! macro.
#![recursion_limit = "1024"]


#[cfg(any(test))]
mod tests;


