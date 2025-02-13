use serde::Deserialize;
use std::error::Error;

mod availability;
mod availability_checker;
mod browser_handler;
mod login_info;
mod notifier;

use availability_checker::AvailabilityChecker;
use browser_handler::BrowserHandler;
use login_info::LoginInfo;
use notifier::DiscordNotifier;

const AVAILABLE_STATUS: &str = "status1";
const DATE_FORMAT: &str = "%-m月%-d日%-H時";
const ONE_WEEK: usize = 7;
const CLASS_PERIOD: usize = 12;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;
    let browser_handler = BrowserHandler::new(config.login_info)?;

    browser_handler.login()?;

    let mut availability_checker =
        AvailabilityChecker::new(browser_handler, config.discord_notifier)?;

    loop {
        match availability_checker.check_all_availability().await {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        };
    }
}

#[derive(Deserialize)]
struct Config {
    discord_notifier: DiscordNotifier,
    login_info: LoginInfo,
}

impl Config {
    fn new() -> Result<Self, Box<dyn Error>> {
        let mut current_exe = std::env::current_exe()?;
        current_exe.pop();
        std::env::set_current_dir(current_exe)?;

        let config_str = std::fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&config_str)?;

        Ok(config)
    }
}
