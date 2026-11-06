//! Compile the maintained Slint entry after its runtime/build graph is admitted.
//!
//! Inputs are fixed source paths. Generated Rust stays in Cargo's owned output;
//! translation text, arbitrary filenames and service state are never source code.

/// Compile one source entry with the selected Fluent/software resource profile.
///
/// # Errors
/// Propagates Slint source or owned build-output errors; no product is published
/// after a failed compilation and no network or system configuration is changed.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=ui");
    let configuration = slint_build::CompilerConfiguration::new()
        .with_style("fluent".into())
        .embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
    slint_build::compile_with_config("ui/app.slint", configuration)?;
    Ok(())
}
