use {
    crate::{commands::CommandExec, context::ScillaContext, error::ScillaResult},
    comfy_table::{presets::UTF8_FULL, Cell, Table},
    console::style,
    inquire::{Select, Text},
    std::fmt,
};

/// Commands related to configuration like RPC_URL, KEYPAIR_PATH etc
#[derive(Debug, Clone)]
pub enum ConfigCommand {
    Show,
    Generate,
    Edit,
    GoBack,
}

impl ConfigCommand {
    pub fn spinner_msg(&self) -> &'static str {
        match self {
            ConfigCommand::Show => "Displaying current Scilla configuration…",
            ConfigCommand::Generate => "Generating new Scilla configuration…",
            ConfigCommand::Edit => "Editing existing Scilla configuration…",
            ConfigCommand::GoBack => "Going back…",
        }
    }
}

impl fmt::Display for ConfigCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let command = match self {
            ConfigCommand::Show => "Show ScillaConfig",
            ConfigCommand::Generate => "Generate ScillaConfig",
            ConfigCommand::Edit => "Edit ScillaConfig",
            ConfigCommand::GoBack => "Go Back",
        };
        write!(f, "{command}")
    }
}

impl ConfigCommand {
    pub async fn process_command(&self, _ctx: &ScillaContext) -> ScillaResult<()> {
        match self {
            ConfigCommand::Show => {
                let config_path = dirs::config_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
                    .join("scilla.toml");

                if !config_path.exists() {
                    println!(
                        "{}",
                        style("No Scilla config found. Use 'Generate' to create one.").yellow()
                    );
                    return Ok(CommandExec::Process(()));
                }

                let content = std::fs::read_to_string(&config_path)?;
                let config: toml::Value = toml::from_str(&content)?;

                let mut table = Table::new();
                table.load_preset(UTF8_FULL).set_header(vec![
                    Cell::new("Setting").add_attribute(comfy_table::Attribute::Bold),
                    Cell::new("Value").add_attribute(comfy_table::Attribute::Bold),
                ]);

                if let toml::Value::Table(map) = config {
                    for (key, value) in map {
                        table.add_row(vec![
                            Cell::new(&key),
                            Cell::new(value.to_string().trim_matches('"')),
                        ]);
                    }
                }

                println!("\n{}", style("SCILLA CONFIGURATION").green().bold());
                println!("{table}");
            }
            ConfigCommand::Generate => {
                let rpc_preset = Select::new(
                    "Select RPC endpoint:",
                    vec!["Devnet", "Testnet", "Mainnet-Beta", "Custom"],
                )
                .prompt()?;

                let rpc_url = match rpc_preset {
                    "Devnet" => "https://api.devnet.solana.com".to_string(),
                    "Testnet" => "https://api.testnet.solana.com".to_string(),
                    "Mainnet-Beta" => "https://api.mainnet-beta.solana.com".to_string(),
                    _ => Text::new("Enter custom RPC URL:").prompt()?,
                };

                let keypair_path = Text::new("Keypair path:")
                    .with_default("~/.config/solana/id.json")
                    .prompt()?;

                let commitment = Select::new(
                    "Commitment level:",
                    vec!["confirmed", "finalized", "processed"],
                )
                .prompt()?;

                let config_content = format!(
                    "rpc-url = \"{rpc_url}\"\nkeypair-path = \"{keypair_path}\"\ncommitment-level \
                     = \"{commitment}\"\n"
                );

                let config_path = dirs::config_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
                    .join("scilla.toml");

                std::fs::create_dir_all(config_path.parent().unwrap())?;
                std::fs::write(&config_path, config_content)?;

                println!(
                    "{}",
                    style(format!("Config saved to: {}", config_path.display()))
                        .green()
                        .bold()
                );
            }
            ConfigCommand::Edit => {
                let config_path = dirs::config_dir()
                    .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
                    .join("scilla.toml");

                let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());

                std::process::Command::new(&editor)
                    .arg(&config_path)
                    .status()
                    .map_err(|e| anyhow::anyhow!("Failed to open editor '{editor}': {e}"))?;

                println!("{}", style("Config file edited.").green());
            }
            ConfigCommand::GoBack => return Ok(CommandExec::GoBack),
        }

        Ok(CommandExec::Process(()))
    }
}
