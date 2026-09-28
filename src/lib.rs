use std::path::PathBuf;

struct BSLZedExtension {
    downloaded_server_path: Option<PathBuf>
}

const _BSL_LANGUAGE_SERVER_REPO: &str = "1c-syntax/bsl-language-server";
const _DOWNLOADED_SERVER_FOLDER_NAME: &str = "bsl_ls";

impl zed_extension_api::Extension for BSLZedExtension {
    fn new() -> Self {
        Self {
            downloaded_server_path: None
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command>
    {
        Ok(zed_extension_api::Command {
            command: "C:/bsl_ls/bsl-language-server.exe".to_owned(),
            args: vec![],
            env: Default::default(),
        }
            )
    }
}

impl BSLZedExtension {
    fn get_server_path(&mut self) -> Result<PathBuf> {
        if let Some(server_path) = &self.downloaded_server_path {
            if server_path.exists() {
                return Ok(server_path.to_owned());
            }
        }
        self.download_server()
    }

    fn download_server(&mut self, language_server_id: &zed_extension_api::LanguageServerId) -> Result<PathBuf> {
        zed_extension_api::set_language_server_installation_status(language_server_id, &zed_extension_api::LanguageServerInstallationStatus::CheckingForUpdate);
    }
}

zed_extension_api::register_extension!(BSLZedExtension);
