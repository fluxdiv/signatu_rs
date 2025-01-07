use clap::ArgMatches;
use anyhow::Result;

/// Returns value at args.config-path, or default via `dirs::config_dir()`
/// default ex: `/home/alice/.config/sigrs/config.sigrs`
pub fn extract_config_path(args: &ArgMatches) -> Result<String, String> {
    // path to config, default via `dirs::config_dir()` if not provided
    match args.get_one::<String>("config-path") {
        Some(path) => Ok(path.clone()),
        None => {
            dirs::config_dir()
                .map_or_else(
                  || Err(String::from("Config path required")),
                  |config_dir| {
                        // /home/alice/.config
                        let mut c = config_dir.into_os_string();
                        c.push("/sigrs/config.sigrs");
                        if let Some(p) = c.to_str() {
                            Ok(p.to_string())
                        } else {
                            Err(String::from("Config path formatting failed"))
                        }
                    }
                )
        }
    }
}
