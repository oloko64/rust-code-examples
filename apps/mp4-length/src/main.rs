use clap::Parser;

mod arg_parser;

// https://www.youtube.com/watch?v=UxHQ7dW6M2s
// https://www.cimarronsystems.com/wp-content/uploads/2017/04/Elements-of-the-H.264-VideoAAC-Audio-MP4-Movie-v2_0.pdf
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = arg_parser::AppArguments::parse();
    validate_input_file(&args.video_file)?;

    #[allow(clippy::cast_possible_truncation)]
    let new_duration_from_user = (args.duration * 1000.0) as u32;

    let mut video = std::fs::read(&args.video_file)?;
    let (time_scale, duration) = video_size_location_slice(&mut video[..])?;

    // 0x3e8 = 1000 (time_scale), the lower the longer the video, default is 1000
    let new_time_scale = format!("{scale:08X}", scale = args.time_scale);
    // 0x8dee = 36334 (duration)
    let new_duration = format!("{new_duration_from_user:08X}");

    hex::decode_to_slice(new_time_scale, time_scale)?;
    hex::decode_to_slice(new_duration, duration)?;

    // println!("{:?}", new_duration);
    // println!("{:?}", duration);
    let output_file = args.output.unwrap_or_else(|| {
        let mut output = args
            .video_file
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .replace(".mp4", "");
        output.push_str("_modified.mp4");
        std::path::PathBuf::from(output)
    });

    std::fs::write(&output_file, video)?;

    println!("Done! The new video is located at: {output_file:?}");

    Ok(())
}

fn validate_input_file(file: &std::path::Path) -> Result<(), String> {
    match file.extension() {
        Some(extension) => {
            if extension != "mp4" {
                Err("The video file must be an `mp4` file")?;
            }
        }
        None => Err("The video file must be a valid file")?,
    }
    if !file.is_file() {
        Err("The video file must be a valid file")?;
    }
    Ok(())
}

/// Returns the time scale and duration of the video
///
/// The first element of the returned tuple is the `time scale`, the second is the `duration`
fn video_size_location_slice(slice: &mut [u8]) -> Result<(&mut [u8], &mut [u8]), String> {
    let mvhd_location = slice
        .windows(4)
        .position(|window| window == b"mvhd")
        .ok_or("`mvhd` not found in video byte data, make sure the video is a valid `mp4`")?;

    // 6d, 76, 68, 64 -> mvhd (6D766864)
    // mvhd == 4 bytes
    // size_position starts 12 bytes after mvhd
    let size_position = mvhd_location + 4 + 12;
    // println!("{:x?}", &slice[mvhd_location..mvhd_location + 4]);

    let size_location = &mut slice[size_position..size_position + 8];

    // [0, 0, 3, e8, 0, 0, 8d, ee]
    // println!("{:x?}", size_location);

    Ok(size_location.split_at_mut(4))
}
