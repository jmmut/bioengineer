use crate::screen::drawing_state::assets::{extract_images, PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH};
use macroquad::texture::{FilterMode, Texture2D};

pub async fn load_tileset(path: &str) -> Vec<Texture2D> {
    let image_future = macroquad::texture::load_image(path);
    let image = image_future.await.unwrap();
    let images = extract_images(&image, PIXELS_PER_TILE_WIDTH, PIXELS_PER_TILE_HEIGHT);
    images
        .into_iter()
        .map(|image| {
            let texture = Texture2D::from_rgba8(image.width, image.height, &image.bytes);
            texture.set_filter(FilterMode::Nearest);
            texture
        })
        .collect()
}
