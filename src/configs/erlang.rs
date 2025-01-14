use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ErlangConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for ErlangConfig<'a> {
    fn default() -> Self {
        ErlangConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: " ",
            style: "bold red",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["rebar.config", "erlang.mk"],
            detect_folders: vec![],
        }
    }
}
