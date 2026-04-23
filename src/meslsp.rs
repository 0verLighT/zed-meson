// https://codeberg.org/0verlight/meslsp

use std::path::PathBuf;
use zed_extension_api::{
    self as zed, LanguageServerId, LanguageServerInstallationStatus as LSPStatus, Result,
};

use crate::utils::file_exists;

pub const LANGUAGE_SERVER_ID: &str = "meslsp";

static MESLSP_VERSION_TAG: &str = "v0.1.0";

pub fn install_or_find_meslsp(id: &LanguageServerId) -> Result<String, String> {
    let (platform, arch) = zed::current_platform();
    
    let arch_tag = match arch {
        zed::Architecture::Aarch64 => "aarch64",
        zed::Architecture::X8664 => "x86_64",
        zed::Architecture::X86 => "x86_64",
    };
    
    let platform_tag = match platform {
        zed::Os::Linux => "unknown-linux-musl",
        zed::Os::Windows => "pc-windows-gnu",
        zed::Os::Mac => "apple-darwin",
    };
    
    let download_dir_name = format!(
        "meslsp-{}-{}-{}",
        MESLSP_VERSION_TAG, arch_tag, platform_tag,
    );
    let bin_path = format!("{}/meslsp", download_dir_name);

    if !file_exists(&PathBuf::from(&bin_path)) {
        // TODO: when the release is available make download logic
        return Err("meslsp: no version available at the moment (Soon...)".to_owned());
    }
    
    Ok(bin_path)
}