fn split_file(
    reader: impl std::io::Read,
    file_name: &str,
    chunk_size: usize,
) -> std::io::Result<()> {
    let dir_path = std::path::Path::new("./out");

    let mut buf = vec![0; chunk_size];
    let mut reader = reader;
    let mut counter = 0;

    let path_to_file = std::path::Path::new(file_name);
    create_dir_all(dir_path)?;

    let file_name = path_to_file.file_name().unwrap().to_str().unwrap();
    loop {
        let n = reader.read(&mut buf)?;

        if n == 0 {
            break;
        }

        counter += 1;
        let full_path = dir_path.join(format!("{:08}-{}", counter, file_name));
        let file = File::create(full_path)?;
        let mut buffered_file = std::io::BufWriter::new(file);

        buffered_file.write_all(&buf[..n])?;
    }

    Ok(())
}

fn merge_files(chunks_path_dir: &str, out_file: &str) -> std::io::Result<()> {
    let mut write_file = BufWriter::new(File::create(out_file)?);
    let mut files = std::fs::read_dir(chunks_path_dir)?
        .filter_map(|res| {
            res.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    files.sort();

    for path in files {
        if path.is_file() {
            let mut file = File::open(path)?;
            let mut buf = vec![];
            let n = file.read_to_end(&mut buf)?;

            if n == 0 {
                return Err(std::io::Error::other("Failed to read file, no bytes read"));
            }

            write_file.write_all(&buf[..n])?;
        }
    }

    Ok(())
}

pub fn example() {
    let file = BufReader::new(File::open("./replace_placeholders.png").unwrap());
    split_file(file, "replace_placeholders.png", 1024).unwrap();

    merge_files("./out", "./replace_placeholders-out.png").unwrap();
}
