use {
    anyhow::Error,
    boilerplate::Boilerplate,
    clap::Parser,
    html_escaper::Escape,
    image::{Rgba, RgbaImage},
    rand::prelude::SliceRandom,
    std::{fs, path::PathBuf},
};

#[derive(Debug, Parser)]
pub struct Arguments {
    pub layers: Vec<PathBuf>,
}

#[derive(Boilerplate)]
struct IndexHtml {
    images: Vec<PathBuf>,
}

type Result<T = (), E = Error> = std::result::Result<T, E>;

fn main() -> Result {
    let arguments = Arguments::parse();

    let mut rng = rand::thread_rng();
    let mut layers = Vec::new();
    for layer in arguments.layers {
        layers.push(image::open(layer)?.into_rgba8());
    }

    fs::remove_dir_all("output").ok();
    fs::create_dir("output")?;

    let mut images = Vec::new();

    for i in 0..100 {
        let mut image = RgbaImage::from_pixel(24, 24, Rgba([0, 0, 0, 255]));

        for _ in 0..8 {
            let layer = layers.choose(&mut rng).unwrap();

            for (x, y, pixel) in layer.enumerate_pixels() {
                if pixel.0 == [255, 255, 255, 255] {
                    image.put_pixel(x, y, *pixel);
                }
            }
        }

        let path = PathBuf::from(format!("output/{i}.png"));

        image.save(&path)?;

        images.push(path);
    }
    fs::write("index.html", IndexHtml { images }.to_string())?;
    Ok(())
}
