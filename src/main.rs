use std::{path::PathBuf, str::FromStr};

use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, Url},
        Request,
    },
    register_plugin, Http, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![DocumentFilter {
        // lsp language id
        language: Some(String::from("PHP")),
        // glob pattern
        pattern: Some(String::from("**/*.php")),
        // like file:
        scheme: None,
    }];
    let mut server_args = vec![];

    // Check for user specified LSP server path
    // ```
    // [lapce-plugin-name.lsp]
    // serverPath = "[path or filename]"
    // serverArgs = ["--arg1", "--arg2"]
    // ```
    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(lsp) = options.get("lsp") {
            if let Some(args) = lsp.get("serverArgs") {
                if let Some(args) = args.as_array() {
                    if !args.is_empty() {
                        server_args = vec![];
                    }
                    for arg in args {
                        if let Some(arg) = arg.as_str() {
                            server_args.push(arg.to_string());
                        }
                    }
                }
            }

            if let Some(server_path) = lsp.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        let server_uri = Url::parse(&format!("urn:{}", server_path))?;
                        PLUGIN_RPC.start_lsp(
                            server_uri,
                            server_args,
                            document_selector,
                            params.initialization_options,
                        );
                        return Ok(());
                    }
                }
            }
        }
    }

    let phan_archive_name = "phan.phar";
    let phan_archive_path = PathBuf::from(phan_archive_name);
    let phan_download_url = format!(
        "https://github.com/phan/phan/releases/latest/download/{}",
        phan_archive_name
    );

    if !phan_archive_path.exists() {
        let mut response = Http::get(&phan_download_url)?;
        let body = response.body_read_all()?;
        std::fs::write(&phan_archive_path, body)?;
    }

    // Architecture check
    let _ = match VoltEnvironment::architecture().as_deref() {
        Ok("x86_64") => "x86_64",
        Ok("aarch64") => "aarch64",
        _ => return Ok(()),
    };

    // OS check
    let _ = match VoltEnvironment::operating_system().as_deref() {
        Ok("macos") => "macos",
        Ok("linux") => "linux",
        Ok("windows") => "windows",
        _ => return Ok(()),
    };

    // Download URL
    // let _ = format!("https://github.com/<name>/<project>/releases/download/<version>/{filename}");

    // see lapce_plugin::Http for available API to download files

    let _ = match VoltEnvironment::operating_system().as_deref() {
        Ok("windows") => {
            format!("{}.exe", "[filename]")
        }
        _ => "[filename]".to_string(),
    };

    // Plugin working directory
    let volt_uri = VoltEnvironment::uri()?;
    let base_path = Url::parse(&volt_uri)?;

    let phan = base_path.join(&format!("{phan_archive_name}"))?;
    
    let php = Url::from_str("php")?;
    
    server_args.push(String::from(phan));

    // if you want to use server from PATH
    // let server_path = Url::parse(&format!("urn:{filename}"))?;

    // Available language IDs
    // https://github.com/lapce/lapce/blob/HEAD/lapce-proxy/src/buffer.rs#L173
    PLUGIN_RPC.start_lsp(
        php,
        server_args,
        document_selector,
        params.initialization_options,
    );

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(params) {
                    PLUGIN_RPC.stderr(&format!("plugin returned with error: {e}"))
                }
            }
            _ => {}
        }
    }
}
