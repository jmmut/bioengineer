use crate::drawing::assets::{extract_images, PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use futures::executor::block_on;
use macroquad::texture::Texture2D;

pub fn load_tileset(path: &str) -> Vec<Texture2D> {
    let image_future = macroquad::texture::load_image(path);
    let image = block_on(image_future).unwrap();
    let images = extract_images(&image, PIXELS_PER_TILE_WIDTH, PIXELS_PER_TILE_HEIGHT);
    images
        .into_iter()
        .map(|image| Texture2D::from_rgba8(image.width, image.height, &image.bytes))
        .collect()
}
