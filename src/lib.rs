#![forbid(unsafe_code)]
// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
// anvomidaviser library — Public API for ISU notation parsing, validation,
// and formal program generation. Use this crate as a library to integrate
// ISU figure skating program analysis into other tools.

pub mod abi;
pub mod codegen;
pub mod manifest;

pub use manifest::{load_manifest, validate, Manifest};

/// Parse, validate, and generate a formal program description from a manifest file.
///
/// This is the high-level entry point for library consumers. It loads the
/// manifest, validates it, and writes generated artifacts to the output directory.
///
/// # Errors
/// Returns an error if the manifest is invalid, elements cannot be parsed,
/// or output files cannot be written.
pub fn generate(manifest_path: &str, output_dir: &str) -> anyhow::Result<()> {
    let m = load_manifest(manifest_path)?;
    validate(&m)?;
    codegen::generate_all(&m, output_dir)
}
