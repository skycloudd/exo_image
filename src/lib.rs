use base64::{engine::general_purpose, Engine as _};
use exolvl::{
    load_exolvl, save_exolvl, Color, ExoLvl, ObjectTile, ObjectTileId, Offset, Pos, Property, Theme,
};
use flate2::{write::GzEncoder, Compression};
use image::{imageops::FilterType, io::Reader, DynamicImage, GenericImageView};
use std::io::{Cursor, Write};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(
    image_data_url: &str,
    should_resize: bool,
    level_name: &str,
) -> Result<Vec<u8>, String> {
    let input = include_str!("../default.json");

    let img = image_data_url
        .split(',')
        .nth(1)
        .ok_or("invalid image data url")?;

    let img = general_purpose::STANDARD
        .decode(img)
        .map_err(|e| e.to_string())?;

    let img = Reader::new(Cursor::new(img))
        .with_guessed_format()
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let mut level = load_exolvl(input).map_err(|e| e.to_string())?;

    let created_time = chrono::Utc::now();

    level.local_level.id = Uuid::new_v4();
    level.local_level.name = level_name.to_string();
    level.local_level.creation_date = created_time;
    level.local_level.update_date = created_time;

    process_image(&mut level, &img, should_resize);

    let output = save_exolvl(&level).map_err(|e| e.to_string())?;

    let mut e = GzEncoder::new(Vec::new(), Compression::default());

    e.write_all(output.as_bytes()).map_err(|e| e.to_string())?;

    Ok(e.finish().map_err(|e| e.to_string())?)
}

fn process_image(level: &mut ExoLvl, img: &DynamicImage, should_resize: bool) {
    let img = if should_resize {
        img.resize(201, 134, FilterType::Lanczos3)
    } else {
        img.clone()
    };

    let pixels = img.pixels();

    let mut entity_id = 0;

    for pixel in pixels {
        let tile = ObjectTile {
            pos: Pos {
                x: pixel.0,
                y: img.height() - pixel.1,
            },
            tile_id: ObjectTileId::Sprite.into(),
            entity_id,
            offset: Offset { x: 0., y: 0. },
            properties: vec![
                Property {
                    name: "sprite".to_string(),
                    value: "colors_decoration#white".to_string(),
                },
                Property {
                    name: "tint".to_string(),
                    value: format!(
                        "#{:02x}{:02x}{:02x}{:02x}",
                        pixel.2[0], pixel.2[1], pixel.2[2], pixel.2[3]
                    ),
                },
            ],
        };

        level.level_data.object_tiles.push(tile);

        entity_id += 1;
    }

    level.level_data.theme = Theme::Custom;
    level.level_data.custom_terrain_color = Color {
        r: 1.,
        g: 1.,
        b: 1.,
        a: 1.,
    };
    level.level_data.custom_background_color = Color {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 1.,
    };
}
