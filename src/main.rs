#![forbid(unsafe_code)]

#[path = "../generated/rust/env.rs"]
mod env;
// This generated flags-2-env module is immutable; scope the generator's
// legacy match-like-matches warning to it, not to hand-written source.
#[allow(clippy::match_like_matches_macro)]
#[path = "../generated/rust/runtime.rs"]
mod env_runtime;

use ores_otel_sidecar::{runtime, SidecarConfig, SidecarIdentity};

fn main() {
    let values = env_runtime::load_from_os();
    let cfg = match SidecarConfig::from_bind(
        SidecarIdentity::new(env::SERVICE, env::BIND),
        &values.bind,
        false,
    ) {
        Ok(cfg) => cfg,
        Err(_) => {
            eprintln!("invalid DECLMIG_SIDECAR_BIND; refusing to start");
            std::process::exit(2);
        }
    };
    runtime::run(&cfg);
}
