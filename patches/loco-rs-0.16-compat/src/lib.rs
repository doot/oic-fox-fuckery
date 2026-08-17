// Compatibility shim: satisfies loco-openapi's `loco-rs = "0.16"` requirement
// by re-exporting the real loco-rs 1.0 crate, so only one physical copy of
// loco-rs (and its types) ends up in the build. Delete this once
// loco-openapi publishes a release compatible with loco-rs 1.0.
// https://github.com/loco-rs/loco-openapi-Initializer
pub use loco_rs::*;
