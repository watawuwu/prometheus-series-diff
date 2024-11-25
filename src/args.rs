use chrono::{DateTime, Duration, Local};
use clap::builder::{styling, Styles};
use clap::{Parser, ValueEnum};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::sync::LazyLock;
use strum::AsRefStr;
use url::Url;

static DEFAULT_START_DATETIME: LazyLock<DateTime<Local>> =
    LazyLock::new(|| Local::now() - Duration::minutes(10));

static DEFAULT_END_DATETIME: LazyLock<DateTime<Local>> =
    LazyLock::new(|| Local::now() - Duration::minutes(5));

fn help_styles() -> Styles {
    styling::Styles::styled()
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}

#[derive(Parser, Debug)]
#[command(author, version, about, next_line_help = true, long_about = None, styles(help_styles()))]
pub struct Args {
    /// Output format
    #[arg(long, default_value_t = String::from("/api/v1/series"))]
    pub api_path: String,

    /// Output format
    #[arg(short, long, default_value_t = OutputFormat::Text)]
    pub output: OutputFormat,

    #[arg(long, default_value_t = *DEFAULT_START_DATETIME)]
    pub from_start: DateTime<Local>,

    #[arg(long, default_value_t = *DEFAULT_END_DATETIME)]
    pub from_end: DateTime<Local>,

    #[arg(long, default_value_t = *DEFAULT_START_DATETIME)]
    pub to_start: DateTime<Local>,

    #[arg(long, default_value_t = *DEFAULT_END_DATETIME)]
    pub to_end: DateTime<Local>,

    #[arg(name = "FROM_INPUT")]
    pub from_input: InputPath,

    #[arg(name = "TO_INPUT")]
    pub to_input: InputPath,
}

#[derive(Debug, Clone)]
pub enum InputPath {
    File(PathBuf),
    Url(Url),
}

impl std::str::FromStr for InputPath {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Try to parse as a URL
        if let Ok(url) = input.parse::<Url>() {
            return Ok(InputPath::Url(url));
        }
        // If not a valid URL, assume it's a file path
        Ok(InputPath::File(PathBuf::from(input)))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum OutputFormat {
    Text,
    Json,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
