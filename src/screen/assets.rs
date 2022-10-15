use crate::Image;

pub const PIXELS_PER_TILE_WIDTH: u16 = 64;
pub const PIXELS_PER_TILE_HEIGHT: u16 = 64;

/// Components refers to each color channel in a pixel, usually r,g,b,a
pub const COMPONENTS_PER_PIXEL: usize = 4;

pub fn extract_images(
    image: &Image,
    pixels_per_tile_width: u16,
    pixels_per_tile_height: u16,
) -> Vec<Image> {
    let tiles_per_tile_line = image.width / pixels_per_tile_width;
    let tiles_per_tile_column = image.height / pixels_per_tile_height;
    let mut images = Vec::new();
    for i_height in 0..tiles_per_tile_column {
        for i_width in 0..tiles_per_tile_line {
            let components = get_components(
                i_width,
                i_height,
                image,
                tiles_per_tile_line,
                tiles_per_tile_column,
                pixels_per_tile_width,
                pixels_per_tile_height,
            );
            images.push(Image {
                width: pixels_per_tile_width,
                height: pixels_per_tile_height,
                bytes: components,
            });
        }
    }
    images
}

fn get_components(
    tile_index_width: u16,
    tile_index_height: u16,
    atlas: &Image,
    tiles_per_tile_line: u16,
    _tiles_per_tile_column: u16,
    pixels_per_tile_width: u16,
    pixels_per_tile_height: u16,
) -> Vec<u8> {
    let mut components = Vec::new();
    let components_per_tile =
        (pixels_per_tile_width * pixels_per_tile_height) as usize * COMPONENTS_PER_PIXEL;
    let components_per_tile_line = components_per_tile * tiles_per_tile_line as usize;
    let components_per_line =
        (pixels_per_tile_width * tiles_per_tile_line) as usize * COMPONENTS_PER_PIXEL;
    let components_of_line_offset =
        (pixels_per_tile_width * tile_index_width) as usize * COMPONENTS_PER_PIXEL;
    for i_h in 0..pixels_per_tile_height as usize {
        for i_w in 0..pixels_per_tile_width as usize * COMPONENTS_PER_PIXEL {
            let index = components_per_tile_line * tile_index_height as usize
                + components_of_line_offset
                + components_per_line * i_h
                + i_w;
            components.push(atlas.bytes[index as usize]);
        }
    }
    components
}

pub fn crop(
    image: &Vec<u8>,
    image_width_in_pixels: usize,
    image_height_in_pixels: usize,
    crop_start_width_in_pixels: usize,
    crop_start_height_in_pixels: usize,
    crop_end_width_in_pixels: usize,
    crop_end_height_in_pixels: usize,
) -> Vec<u8> {
    assert_eq!(
        image.len(),
        image_width_in_pixels * image_height_in_pixels * COMPONENTS_PER_PIXEL
    );
    let crop_start_width_in_components = crop_start_width_in_pixels * COMPONENTS_PER_PIXEL;
    let crop_end_width_in_components = crop_end_width_in_pixels * COMPONENTS_PER_PIXEL;
    let components_per_line = image_width_in_pixels * COMPONENTS_PER_PIXEL;
    let mut result = Vec::<u8>::with_capacity(
        (crop_end_width_in_pixels - crop_start_width_in_pixels)
            * (crop_end_height_in_pixels - crop_start_height_in_pixels)
            * COMPONENTS_PER_PIXEL,
    );
    for i_h in crop_start_height_in_pixels..crop_end_height_in_pixels {
        for i_w in crop_start_width_in_components..crop_end_width_in_components {
            let component = image[i_h * components_per_line + i_w];
            result.push(component)
        }
    }
    result
}

/// `factor`: each pixel will be repeated `factor` times, vertically and horizontally
pub fn zoom(image: &Vec<u8>, image_width_in_pixels: usize, factor: usize) -> Vec<u8> {
    let mut result = Vec::<u8>::with_capacity(image.len() * factor * factor);
    let mut line = Vec::<u8>::with_capacity(image_width_in_pixels * factor * COMPONENTS_PER_PIXEL);
    for i_h in 0..image.len() / COMPONENTS_PER_PIXEL / image_width_in_pixels {
        line.clear();
        for i_w in 0..image_width_in_pixels {
            for _i_copy in 0..factor {
                for i_c in 0..COMPONENTS_PER_PIXEL {
                    line.push(
                        image[i_h * image_width_in_pixels * COMPONENTS_PER_PIXEL
                            + i_w * COMPONENTS_PER_PIXEL
                            + i_c],
                    );
                }
            }
        }
        for i_copy in 0..factor {
            result.append(&mut line.clone());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_pixels() {
        #[rustfmt::skip]
        let components = vec![
            10, 11, 12, 13, 14, 15, 16, 17,
            20, 21, 22, 23, 24, 25, 26, 27,
            30, 31, 32, 33, 34, 35, 36, 37,
            40, 41, 42, 43, 44, 45, 46, 47,
            50, 51, 52, 53, 54, 55, 56, 57,
            60, 61, 62, 63, 64, 65, 66, 67,
            70, 71, 72, 73, 74, 75, 76, 77,
            80, 81, 82, 83, 84, 85, 86, 87,
        ];
        let image = Image {
            width: 2,
            height: 8,
            bytes: components,
        };
        let components = get_components(0, 0, &image, 2, 2, 1, 4);
        assert_eq!(components.len(), 16);
        #[rustfmt::skip]
        assert_eq!(components, vec![
            10, 11, 12, 13,
            20, 21, 22, 23,
            30, 31, 32, 33,
            40, 41, 42, 43,
        ]);

        let components = get_components(1, 0, &image, 2, 2, 1, 4);
        assert_eq!(components.len(), 16);
        #[rustfmt::skip]
        assert_eq!(components, vec![
            14, 15, 16, 17,
            24, 25, 26, 27,
            34, 35, 36, 37,
            44, 45, 46, 47,
        ]);

        let components = get_components(0, 1, &image, 2, 2, 1, 4);
        assert_eq!(components.len(), 16);
        #[rustfmt::skip]
        assert_eq!(components, vec![
            50, 51, 52, 53,
            60, 61, 62, 63,
            70, 71, 72, 73,
            80, 81, 82, 83,
        ]);

        let components = get_components(1, 1, &image, 2, 2, 1, 4);
        assert_eq!(components.len(), 16);
        #[rustfmt::skip]
        assert_eq!(components, vec![
            54, 55, 56, 57,
            64, 65, 66, 67,
            74, 75, 76, 77,
            84, 85, 86, 87,
        ]);

        let components = get_components(1, 3, &image, 2, 4, 1, 2);
        assert_eq!(components.len(), 8);
        #[rustfmt::skip]
        assert_eq!(components, vec![
            74, 75, 76, 77,
            84, 85, 86, 87,
        ]);

        let textures = extract_images(&image, 1, 4);
        assert_eq!(textures.len(), 4);
    }

    #[test]
    fn test_crop() {
        #[rustfmt::skip]
        let image: Vec<u8> = vec![
            0, 1, 2, 3,       4, 5, 6, 7,       8, 9, 10, 11,     12, 13, 14, 15,
            16, 17, 18, 19,   20, 21, 22, 23,   24, 25, 26, 27,   28, 29, 30, 31,
            32, 33, 34, 35,   36, 37, 38, 39,   40, 41, 42, 43,   44, 45, 46, 47,
            48, 49, 50, 51,   52, 53, 54, 55,   56, 57, 58, 59,   60, 61, 62, 63
        ];
        let cropped = crop(&image, 4, 4, 1, 1, 3, 3);
        let expected: Vec<u8> = vec![
            20, 21, 22, 23, 24, 25, 26, 27, 36, 37, 38, 39, 40, 41, 42, 43,
        ];
        assert_eq!(cropped, expected);
    }

    #[test]
    fn test_zoom() {
        let image: Vec<u8> = vec![
            20, 21, 22, 23, 24, 25, 26, 27, 36, 37, 38, 39, 40, 41, 42, 43,
        ];
        let zoomed = zoom(&image, 2, 2);
        let expected: Vec<u8> = vec![
            20, 21, 22, 23, 20, 21, 22, 23, 24, 25, 26, 27, 24, 25, 26, 27, 20, 21, 22, 23, 20, 21,
            22, 23, 24, 25, 26, 27, 24, 25, 26, 27, 36, 37, 38, 39, 36, 37, 38, 39, 40, 41, 42, 43,
            40, 41, 42, 43, 36, 37, 38, 39, 36, 37, 38, 39, 40, 41, 42, 43, 40, 41, 42, 43,
        ];
        assert_eq!(zoomed, expected);
    }
}
