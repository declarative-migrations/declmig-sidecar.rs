#![forbid(unsafe_code)]

#[path = "../generated/rust/env.rs"]
mod env;
// This generated flags-2-env module is immutable; scope the generator's
// legacy match-like-matches warning to it, not to hand-written source.
#[allow(clippy::match_like_matches_macro)]
#[path = "../generated/rust/runtime.rs"]
mod env_runtime;

use ores_otel_sidecar::{health, SidecarConfig, SidecarIdentity};

#[test]
fn inherits_shared_health() {
    let identity = SidecarIdentity::new(env::SERVICE, env::BIND);
    let payload = health::current(identity, None);
    assert!(payload.ok);
    assert_eq!(payload.service, env::SERVICE);
}

#[test]
fn runtime_falls_back_to_catalog_default() {
    let values = env_runtime::load_from(|_| None);
    assert_eq!(values.bind, env::BIND_DEFAULT);
}

#[test]
fn invalid_bind_is_rejected_without_fallback() {
    let identity = SidecarIdentity::new(env::SERVICE, env::BIND);
    assert!(SidecarConfig::from_bind(identity, "<not-an-address>", false).is_err());
}
