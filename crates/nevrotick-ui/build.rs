//! Compile the maintained Slint entry after its runtime/build graph is admitted.
//!
//! Inputs are fixed source paths. Generated Rust stays in Cargo's owned output.
//! The configured Windows runner supplies the selected, immutable version resource.

use std::env::{self, VarError};
use std::fs::{self, Metadata};
use std::io::{self, ErrorKind};

/// Link the normal UI manifest and the runner's optional version resource.
///
/// The runner binds the selection digest to the resource bytes and keeps the
/// resource immutable through linking. Cargo tracks both inputs across builds.
///
/// # Errors
/// Rejects an unavailable Cargo target, a malformed selection, or a selected
/// resource that is inaccessible, nonregular, empty, or larger than 4096 bytes.
fn link_windows_resources() -> io::Result<()> {
    const SELECTION: &str = "NEVROTICK_UI_VERSIONINFO_SHA256";
    const RESOURCE: &str = "/work/metadata/ui.res";

    let target: String = env::var("TARGET").map_err(|_: VarError| {
        io::Error::new(ErrorKind::InvalidInput, "unavailable Cargo target")
    })?;
    if target != "x86_64-pc-windows-msvc" {
        return Ok(());
    }

    // Link the approved ordinary UI execution level and UIAccess declaration.
    println!("cargo::rustc-link-arg-bin=nevrotick-ui=/MANIFEST:EMBED,ID=1");
    println!(
        "cargo::rustc-link-arg-bin=nevrotick-ui=/MANIFESTUAC:level='asInvoker' uiAccess='false'"
    );

    println!("cargo::rerun-if-env-changed={SELECTION}");
    let digest: String = match env::var(SELECTION) {
        Ok(value) => value,
        Err(VarError::NotPresent) => return Ok(()),
        Err(VarError::NotUnicode(_)) => {
            return Err(io::Error::new(
                ErrorKind::InvalidInput,
                "UI VERSIONINFO selection must be Unicode",
            ));
        }
    };
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|byte: u8| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "UI VERSIONINFO selection must be a lowercase SHA-256 digest",
        ));
    }

    println!("cargo::rerun-if-changed={RESOURCE}");
    let resource: Metadata = fs::symlink_metadata(RESOURCE)?;
    if !resource.is_file() || !(1..=4096).contains(&resource.len()) {
        return Err(io::Error::new(
            ErrorKind::InvalidData,
            "UI VERSIONINFO must be a regular resource file of 1 to 4096 bytes",
        ));
    }
    println!("cargo::rustc-link-arg-bin=nevrotick-ui={RESOURCE}");
    Ok(())
}

/// Compile one source entry with the selected Fluent/software resource profile.
///
/// # Errors
/// Propagates selected version resource, Slint source, or build-output errors.
/// A failed build publishes no product artifact.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=ui");
    link_windows_resources()?;
    let configuration: slint_build::CompilerConfiguration =
        slint_build::CompilerConfiguration::new()
            .with_style("fluent".into())
            .embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
    slint_build::compile_with_config("ui/app.slint", configuration)?;
    Ok(())
}
