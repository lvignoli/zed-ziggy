use zed_extension_api::{self as zed, Result};

struct ZiggyExtension {
    cached_binary_path: Option<String>,
}

impl ZiggyExtension {
    fn language_server_binary(
        &mut self,
        _: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        worktree
            .which("ziggy")
            .ok_or("ziggy executable not found in worktree path".to_owned())

        // TODO(lvignoli): Uncomment when zed-industries/zed#121407 is fixed.
        // https://github.com/zed-industries/zed/issues/21407

        // if let Some(path) = worktree.which("ziggy") {
        //     return Ok(path);
        // }

        // if let Some(path) = &self.cached_binary_path {
        //     if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
        //         return Ok(path.clone());
        //     }
        // }

        // zed::set_language_server_installation_status(
        //     language_server_id,
        //     &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        // );
        // let release = zed::latest_github_release(
        //     "kristoff-it/ziggy",
        //     zed::GithubReleaseOptions {
        //         require_assets: true,
        //         pre_release: true,
        //     },
        // )?;

        // let (platform, arch) = zed::current_platform();
        // let asset_name = format!(
        //     "{arch}-{os}.tar.xz",
        //     arch = match arch {
        //         zed::Architecture::Aarch64 => "aarch64",
        //         zed::Architecture::X86 => return Err("x86 not supported".to_string()),
        //         zed::Architecture::X8664 => "x86_64",
        //     },
        //     os = match platform {
        //         zed::Os::Mac => "macos",
        //         zed::Os::Linux => "linux-musl",
        //         zed::Os::Windows => "windows",
        //     },
        // );

        // let asset = release
        //     .assets
        //     .iter()
        //     .find(|asset| asset.name == asset_name)
        //     .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        // let version_dir = format!("ziggy-{}", release.version);
        // let binary_path = format!("{version_dir}/ziggy");

        // if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
        //     zed::set_language_server_installation_status(
        //         language_server_id,
        //         &zed::LanguageServerInstallationStatus::Downloading,
        //     );

        //     zed::download_file(
        //         &asset.download_url,
        //         &version_dir,
        //         zed::DownloadedFileType::GzipTar,
        //     )
        //     .map_err(|e| format!("failed to download file: {e}"))?;

        //     let entries =
        //         fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
        //     for entry in entries {
        //         let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
        //         if entry.file_name().to_str() != Some(&version_dir) {
        //             fs::remove_dir_all(entry.path()).ok();
        //         }
        //     }
        // }

        // self.cached_binary_path = Some(binary_path.clone());
        // Ok(binary_path)
    }
}

impl zed::Extension for ZiggyExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let ziggy_binary = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: ziggy_binary,
            args: vec!["lsp".to_string()],
            env: vec![],
        })
    }
}

zed::register_extension!(ZiggyExtension);
