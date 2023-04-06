use serde::{Deserialize, Serialize};
use crate::openai::response::images::OpenAIImage;
use viuer::Config;
use std::path::PathBuf;
use rand::Rng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIImageEditResponse {
    data: Vec<OpenAIImage>,
}

impl OpenAIImageEditResponse {
	pub fn print_image_url(self) {
		trace!("print images");
		for img in &self.data {
			println!("{}", img.url.to_owned().unwrap());
		}
        if self.data.is_empty() {
            debug!("No images were created");
        }
	}

	pub fn print_image_b64(self) {
		trace!("print images");
		for img in &self.data {
			println!("{}", img.b64_json.to_owned().unwrap());
		}
        if self.data.is_empty() {
            debug!("No images were created");
        }
	}

	pub fn save_images(self, size: String, mut out_path: PathBuf) {
		trace!("save images to {:#?}", out_path.to_string_lossy());
        std::fs::create_dir_all(&out_path).unwrap();
        let out_path = match &out_path.is_file() {
            true => out_path,
            false => {
                let mut rng = rand::thread_rng();
                let filename: u16 = rng.gen();
                match &out_path.is_dir() {
                    true => {
                        out_path.push(format!("{:#?}.png", filename));
                        out_path
                    }
                    false => {
                        let output = format!("{:#?}.png", filename);
                        PathBuf::from(&output)
                    }
                }
            },
        };
        let size_range = size.split('x').collect::<Vec<&str>>();
        let _x:u32 = size_range[0].parse::<u32>().unwrap();
        let _y:u32 = size_range[1].parse::<u32>().unwrap();
		for img in &self.data {
            let image_string = img.b64_json.to_owned().unwrap();
            let decoded_image = base64::decode_config(image_string, base64::STANDARD).unwrap();

            match image::load_from_memory_with_format(&decoded_image, image::ImageFormat::Png) {
                Ok(png) => {
                    debug!("saving png to: {:#?}", out_path);
                    png.to_rgba16().clone().save(&out_path).unwrap();
                }
                Err(error) => {
                    error!("input is not formatted as expected: {}", error);
                }
            }
		}

        if self.data.is_empty() {
            debug!("No images to create");
        }
	}

	pub fn print_images(self, size: String) {
		trace!("print images");
        let size_range = size.split('x').collect::<Vec<&str>>();
        let _x:u32 = size_range[0].parse::<u32>().unwrap();
        let _y:u32 = size_range[1].parse::<u32>().unwrap();
		for img in &self.data {
            let conf = Config {
                x: 0,
                y: size_range[1].parse::<i16>().unwrap(),
                ..Default::default()
            };
            let image_string = img.b64_json.to_owned().unwrap();
            let decoded_image = base64::decode_config(image_string, base64::STANDARD).unwrap();

            match image::load_from_memory(&decoded_image) {
                Ok(img) => {
                    viuer::print(&img, &conf).expect("Image printing failed.");
                },
                Err(error) => {
                    error!("input is not formatted as expected: {}", error);
                }
            }
		}

        if self.data.is_empty() {
            debug!("No images to print");
        }
	}
}
