use image::ImageOutputFormat;
use imageproc::drawing::text_size;
use rusttype::{Font, Scale};
use serenity::prelude::TypeMapKey;
use std::{io::Cursor, sync::Arc};

pub struct ImageGenerator<'a> {
    pub player_font: Font<'a>,
    pub text_font: Font<'a>,
    pub teams_image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
}

impl<'a> ImageGenerator<'a> {
    pub fn draw_teams_to_vec(&self, player_names: Vec<String>, teams_rating: [i32; 2], format: ImageOutputFormat) -> Vec<u8> {
        let mut image = self.teams_image.clone();

        let player_text_scale = Scale::uniform(60.0);
        for i in 0..2 {
            for j in 0..5 {
                let player_name = match player_names.get(i * 5 + j) {
                    Some(name) => name,
                    None => "Unknown",
                };

                let size = text_size(player_text_scale, &self.player_font, player_name);
                let scale = if size.0 > 340 {
                    Scale::uniform(340.0 / size.0 as f32 * player_text_scale.x)
                } else {
                    player_text_scale
                };
                let size = text_size(scale, &self.player_font, player_name);

                let x: i32 = 83 + 540 * i as i32 - 2;
                let y: i32 =
                    182 + 70 * j as i32 - size.1 + ((size.1 as f32 * 1.0 / 5.0) / 10.0) as i32;

                imageproc::drawing::draw_text_mut(
                    &mut image,
                    image::Rgb([255, 255, 255]),
                    x,
                    y,
                    scale,
                    &self.player_font,
                    player_name,
                );
            }
        }

        let rating_text_scale = Scale::uniform(86.5);
        for (i, rating) in teams_rating.iter().enumerate() {
            let rating = rating.to_string();

            let size = text_size(rating_text_scale, &self.player_font, &rating);
            imageproc::drawing::draw_text_mut(
                &mut image,
                image::Rgb([255, 255, 255]),
                370 - size.0 / 2 + 540 * i as i32,
                100 - size.1,
                rating_text_scale,
                &self.text_font,
                &rating,
            );
        }

        
        let mut buf = Cursor::new(Vec::new());
        image.write_to(&mut buf, format).unwrap();

        buf.into_inner()
    }
}

pub struct ImageGeneratorContainer;

impl TypeMapKey for ImageGeneratorContainer {
    type Value = Arc<ImageGenerator<'static>>;
}
