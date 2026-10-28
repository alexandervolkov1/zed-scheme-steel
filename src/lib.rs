use zed_extension_api as zed;

struct SteelExtension;

impl zed::Extension for SteelExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let command = worktree
            .which("steel-language-server")
            .ok_or_else(|| "steel-language-server not found in PATH".to_string())?;

        Ok(zed::Command {
            command,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SteelExtension);

