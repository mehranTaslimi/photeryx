use anyhow::anyhow;
use img_hash::{HasherConfig, ImageHash, image as ih};

use crate::ops::DOCUMENTS;

struct Image {
    id: u32,
    hash: String,
}

pub fn find_duplicates(ids: Vec<u32>, threshold: u8) -> anyhow::Result<Vec<Vec<u32>>> {
    let hasher = HasherConfig::new().to_hasher();

    let images = ids
        .iter()
        .filter_map(|id| DOCUMENTS.get(id))
        .filter_map(|image| {
            let rgba = image.to_rgba8();

            let width = rgba.width();
            let height = rgba.height();

            let buf = rgba.into_raw();

            let maybe_raw = ih::RgbaImage::from_raw(width, height, buf);

            match maybe_raw {
                Some(raw) => {
                    let hash = hasher.hash_image(&raw).to_base64();
                    Some(Image {
                        id: *image.key(),
                        hash,
                    })
                }
                None => None,
            }
        })
        .collect::<Vec<Image>>();

    let mut groups = vec![];
    let mut processed = vec![false; images.len()];

    for i in 0..images.len() {
        if processed[i] {
            continue;
        }

        let mut group = vec![images[i].id];
        processed[i] = true;

        let hash1: ImageHash<Vec<u8>> = ImageHash::from_base64(&images[i].hash)
            .map_err(|_| anyhow!("Invalid hash id: {}", images[i].id))?;

        for j in i + 1..images.len() {
            if processed[j] {
                continue;
            }

            let hash2: ImageHash<Vec<u8>> = ImageHash::from_base64(&images[j].hash)
                .map_err(|_| anyhow!("Invalid hash id: {}", images[i].id))?;

            let distance = hash1.dist(&hash2) as f64;
            let max_distance = 64.0;
            let similarity = 100.0 - ((distance / max_distance) * 100.0);

            if similarity >= threshold as f64 {
                group.push(images[j].id);
                processed[j] = true;
            }
        }

        if group.len() > 1 {
            groups.push(group);
        }
    }

    Ok(groups)
}
