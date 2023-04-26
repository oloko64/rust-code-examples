use clap::Parser;

/// API mock to get tabs and catalog for payments
#[derive(Parser)]
#[command(version)]
pub struct AppArguments {
    /// Path to the file with tabs mock
    #[clap(short, long, default_value = "tab_api_mock.json")]
    pub tabs_path: String,

    /// Path to the file with catalog mock
    #[clap(short, long, default_value = "catalog_api_mock.json")]
    pub catalog_path: String,

    /// Port to run the server on
    #[clap(short, long, default_value_t = 5000)]
    pub port: u16,
}
