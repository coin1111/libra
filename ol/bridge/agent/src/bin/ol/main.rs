//! Main entry point for OlCli

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use bridge_agent::application::APPLICATION;

/// Boot OlCli
fn main() {
    abscissa_core::boot(&APPLICATION);
}
