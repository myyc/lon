use super::conversion::{classify_family, hex_to_rgb, rgb_to_hsl};
use super::model::{ColorLibrary, PantoneColor};
use gtk::gio;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct TcxJson {
    names: Vec<String>,
    values: Vec<String>,
}

#[derive(Deserialize)]
struct SolidCoatedEntry {
    name: String,
    hex: String,
}

fn load_tcx_colors() -> Vec<PantoneColor> {
    let bytes = gio::resources_lookup_data(
        "/dev/myyc/lon/colors/tcx.json",
        gio::ResourceLookupFlags::NONE,
    )
    .expect("Failed to load TCX color data");

    let json_str = std::str::from_utf8(&bytes).expect("Invalid UTF-8 in TCX data");
    let raw: TcxJson = serde_json::from_str(json_str).expect("Failed to parse TCX JSON");

    raw.names
        .into_iter()
        .zip(raw.values)
        .filter_map(|(name, hex)| {
            let rgb = hex_to_rgb(&hex)?;
            let hsl = rgb_to_hsl(&rgb);
            let family = classify_family(&hsl);

            Some(PantoneColor {
                name,
                hex,
                rgb,
                hsl,
                family,
                library: ColorLibrary::FashionHomeTcx,
            })
        })
        .collect()
}

fn load_solid_coated_colors() -> Vec<PantoneColor> {
    let bytes = gio::resources_lookup_data(
        "/dev/myyc/lon/colors/solid_coated.json",
        gio::ResourceLookupFlags::NONE,
    )
    .expect("Failed to load Solid Coated color data");

    let json_str = std::str::from_utf8(&bytes).expect("Invalid UTF-8 in Solid Coated data");
    let raw: Vec<SolidCoatedEntry> =
        serde_json::from_str(json_str).expect("Failed to parse Solid Coated JSON");

    raw.into_iter()
        .filter_map(|entry| {
            let rgb = hex_to_rgb(&entry.hex)?;
            let hsl = rgb_to_hsl(&rgb);
            let family = classify_family(&hsl);

            Some(PantoneColor {
                name: entry.name,
                hex: entry.hex,
                rgb,
                hsl,
                family,
                library: ColorLibrary::SolidCoated,
            })
        })
        .collect()
}

pub struct ColorDatabase {
    colors: HashMap<ColorLibrary, Vec<PantoneColor>>,
}

impl ColorDatabase {
    pub fn new() -> Self {
        let mut colors = HashMap::new();
        colors.insert(ColorLibrary::FashionHomeTcx, load_tcx_colors());
        colors.insert(ColorLibrary::SolidCoated, load_solid_coated_colors());
        Self { colors }
    }

    pub fn get_library(&self, library: ColorLibrary) -> &[PantoneColor] {
        self.colors
            .get(&library)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn library_count(&self, library: ColorLibrary) -> usize {
        self.get_library(library).len()
    }
}

impl Default for ColorDatabase {
    fn default() -> Self {
        Self::new()
    }
}
