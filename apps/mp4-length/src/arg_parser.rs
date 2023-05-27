use clap::Parser;

/// A simple CLI app to mess with `mp4` video lengths
#[derive(Parser)]
#[command(version)]
pub struct AppArguments {
    pub video_file: std::path::PathBuf,

    /// The output file name
    #[arg(short, long)]
    pub output: Option<std::path::PathBuf>,

    /// The time scale of the video to be applied, the lower the longer the video, default is 1000
    #[arg(short = 's', long, default_value_t = 1000)]
    pub time_scale: u32,

    /// The duration of the video in seconds to be applied, generally this is the only value you want to change
    #[arg(short, long)]
    pub duration: f64,
}
