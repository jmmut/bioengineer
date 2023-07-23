use crate::screen::assets::{
    crop, extract_images, zoom, PIXELS_PER_TILE_HEIGHT, PIXELS_PER_TILE_WIDTH,
};
use crate::world::map::cell::ExtraTextures;
use crate::world::map::cell::TextureIndexTrait;
use macroquad::texture::{FilterMode, Texture2D};

pub async fn load_tileset(path: &str) -> Vec<Texture2D> {
    let image_future = macroquad::texture::load_image(path);
    let image = image_future.await.unwrap();
    let images = extract_images(&image, PIXELS_PER_TILE_WIDTH, PIXELS_PER_TILE_HEIGHT);
    let loaded_textures = images
        .into_iter()
        .map(|image| {
            let texture = Texture2D::from_rgba8(image.width, image.height, &image.bytes);
            texture.set_filter(FilterMode::Nearest);
            texture
        })
        .collect();
    compute_extra_textures(loaded_textures)
}

fn compute_extra_textures(textures: Vec<Texture2D>) -> Vec<Texture2D> {
    let textures = add_zoomed_robot(textures);
    textures
}

fn add_zoomed_robot(mut textures: Vec<Texture2D>) -> Vec<Texture2D> {
    let robot = textures[ExtraTextures::Robot.get_index()];
    let image = robot.get_texture_data();
    let subimage_start_width = (PIXELS_PER_TILE_WIDTH / 4) as usize;
    let subimage_start_height = (PIXELS_PER_TILE_HEIGHT / 4) as usize;
    let subimage_end_width = (PIXELS_PER_TILE_WIDTH * 3 / 4) as usize;
    let subimage_end_height = (PIXELS_PER_TILE_HEIGHT * 3 / 4) as usize;
    let cropped = crop(
        &image.bytes,
        PIXELS_PER_TILE_WIDTH as usize,
        PIXELS_PER_TILE_HEIGHT as usize,
        subimage_start_width,
        subimage_start_height,
        subimage_end_width,
        subimage_end_height,
    );
    let zoomed = zoom(&cropped, subimage_end_width - subimage_start_width, 2);
    let texture = Texture2D::from_rgba8(PIXELS_PER_TILE_WIDTH, PIXELS_PER_TILE_HEIGHT, &zoomed);
    texture.set_filter(FilterMode::Nearest);
    textures.push(texture);
    textures
}
