use std::{io::{BufReader, Seek, SeekFrom, Read, BufWriter, BufRead, Write}, fs::File};

use rand::Rng;

fn read_as_bytes() {
    let num_of_ceps = 1205409;
    let mut file = BufReader::new(File::open("file.bin").unwrap());

    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let seek_position = rng.gen_range(0..=num_of_ceps) * 4;

        file.seek(SeekFrom::Start(seek_position)).unwrap();

        let mut buffer = [0; 4];
        file.read_exact(&mut buffer).unwrap();

        println!("{:08}", u32::from_le_bytes(buffer));
    }
}

fn write_as_bytes() {
    let mut file = BufReader::new(File::open("file-sorted.txt").unwrap());
    let mut binary_file = BufWriter::new(File::create("file-sorted.bin").unwrap());

    let mut buffer = String::new();
    let mut res = file.read_line(&mut buffer).unwrap();

    while res != 0 {
        let number = buffer[..8].parse::<u32>().unwrap().to_le_bytes();

        binary_file.write_all(&number).unwrap();
        buffer.clear();
        res = file.read_line(&mut buffer).unwrap();
    }
}

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
