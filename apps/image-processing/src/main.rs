mod image;

fn main() {
    let mut img = image::Image::new_from_path("imgs/rgby.png").unwrap();

    // println!("{:#?}", img);

    img.negative();
    img.blur(5.0);

    img.save_image("imgs/rgby-out.png").unwrap();
}
