use std::{io::{BufReader, Seek, SeekFrom, Read}, fs::File};

use rand::Rng;

fn get_random_cep() {
    let num_of_ceps = 1205409;
    let mut file = BufReader::new(File::open("file-no-new-lines.txt").unwrap());

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let seek_position = rng.gen_range(0..=num_of_ceps) * 8;

        file.seek(SeekFrom::Start(seek_position)).unwrap();

        let mut buffer = [0; 8];
        file.read_exact(&mut buffer).unwrap();

        let cep_string = String::from_utf8(buffer.to_vec()).unwrap();
        println!("{}-{}", &cep_string[0..5], &cep_string[5..8])
    }
}

fn remove_new_lines() {
    let mut file = File::open("file.txt").unwrap();

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    // remove \n from array of bytes
    let data = data.into_iter().filter(|&c| c != 10).collect::<Vec<_>>();

    println!("{:X?}", &data[..8]);
    // dbg!(&data[..8]);

    // let mut new_file = File::create("ceps-number-sorted-no-new-lines.txt").unwrap();
    // new_file.write_all(&data).unwrap();
}
