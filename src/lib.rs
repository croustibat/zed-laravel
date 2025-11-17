use zed_extension_api::{self as zed, Result};
use std::fs;

struct BladeEnhancedExtension {
    cached_laravel_ls_binary: Option<String>,
}

impl zed::Extension for BladeEnhancedExtension {
    fn new() -> Self {
        Self {
            cached_laravel_ls_binary: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "laravel-ls" => {
                let binary_path = self.get_laravel_ls_binary(language_server_id, worktree)?;

                Ok(zed::Command {
                    command: binary_path,
                    args: vec![],
                    env: Default::default(),
                })
            }
            "intelephense" => {
                // Use Intelephense for PHP code within Blade templates
                Ok(zed::Command {
                    command: zed::node_binary_path()?,
                    args: vec![
                        worktree
                            .which("intelephense")
                            .ok_or_else(|| "intelephense not found".to_string())?,
                        "--stdio".to_string(),
                    ],
                    env: Default::default(),
                })
            }
            language_server => Err(format!("unknown language server: {language_server}"))?,
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        if language_server_id.as_ref() == "intelephense" {
            return Ok(Some(zed::serde_json::json!({
                "intelephense": {
                    "files": {
                        "associations": ["*.php", "*.blade.php"],
                        "maxSize": 5000000
                    },
                    "environment": {
                        "phpVersion": "8.2.0"
                    },
                    "stubs": [
                        "apache",
                        "bcmath",
                        "bz2",
                        "calendar",
                        "com_dotnet",
                        "Core",
                        "ctype",
                        "curl",
                        "date",
                        "dba",
                        "dom",
                        "enchant",
                        "exif",
                        "FFI",
                        "fileinfo",
                        "filter",
                        "fpm",
                        "ftp",
                        "gd",
                        "gettext",
                        "gmp",
                        "hash",
                        "iconv",
                        "imap",
                        "intl",
                        "json",
                        "ldap",
                        "libxml",
                        "mbstring",
                        "meta",
                        "mysqli",
                        "oci8",
                        "odbc",
                        "openssl",
                        "pcntl",
                        "pcre",
                        "PDO",
                        "pdo_ibm",
                        "pdo_mysql",
                        "pdo_pgsql",
                        "pdo_sqlite",
                        "pgsql",
                        "Phar",
                        "posix",
                        "pspell",
                        "readline",
                        "Reflection",
                        "session",
                        "shmop",
                        "SimpleXML",
                        "snmp",
                        "soap",
                        "sockets",
                        "sodium",
                        "SPL",
                        "sqlite3",
                        "standard",
                        "superglobals",
                        "sysvmsg",
                        "sysvsem",
                        "sysvshm",
                        "tidy",
                        "tokenizer",
                        "xml",
                        "xmlreader",
                        "xmlrpc",
                        "xmlwriter",
                        "xsl",
                        "Zend OPcache",
                        "zip",
                        "zlib"
                    ]
                }
            })));
        }

        Ok(None)
    }
}

impl BladeEnhancedExtension {
    fn get_laravel_ls_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // Check if we already have a cached binary path
        if let Some(cached) = &self.cached_laravel_ls_binary {
            if fs::metadata(cached).is_ok() {
                return Ok(cached.clone());
            }
        }

        // Try to find laravel-ls in the system PATH
        if let Some(path) = worktree.which("laravel-ls") {
            self.cached_laravel_ls_binary = Some(path.clone());
            return Ok(path);
        }

        // Download laravel-ls from GitHub releases
        let release = zed::latest_github_release(
            "laravel-ls/laravel-ls",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let version = release.version.trim_start_matches('v');
        let (os_type, arch_type) = zed::current_platform();

        let os = match os_type {
            zed::Os::Mac => "darwin",
            zed::Os::Linux => "linux",
            zed::Os::Windows => "windows",
        };

        let arch = match arch_type {
            zed::Architecture::Aarch64 => "arm64",
            zed::Architecture::X8664 => "amd64",
            zed::Architecture::X86 => "386",
        };

        let asset_name = format!("laravel-ls-{}-{}.tar.gz", os, arch);

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("laravel-ls-{}", version);
        fs::create_dir_all(&version_dir)
            .map_err(|e| format!("failed to create directory: {}", e))?;

        let binary_path = format!("{}/laravel-ls", version_dir);

        if !fs::metadata(&binary_path).is_ok() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries =
                fs::read_dir(".")
                    .map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_laravel_ls_binary = Some(binary_path.clone());
        Ok(binary_path)
    }
}

zed::register_extension!(BladeEnhancedExtension);
