use std::{fs, path::Path};


fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = (h % 1.0) * 6.0; // Hue in [0, 6)
    let f = h - h.floor();
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let (r, g, b) = match h as u8 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => (0.0, 0.0, 0.0), // This shouldn't happen due to the modulo
    };

    // Convert to 0-255 range
    (
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    )
}

pub fn generate_colors(n: usize, hue_offset: f32) -> Vec<egui::Color32> {
    let mut colors = Vec::with_capacity(n);

    for i in 0..n {
        let hue = (i as f32 / n as f32 + hue_offset) % 1.0;
        let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0); // Full saturation and value
        colors.push(egui::Color32::from_rgb(r, g, b)); // Convert to Color32
    }

    colors
}

pub fn get_json_files_in_folder(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut json_files = Vec::new();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                json_files.push(file_name.to_string());
            }
        }
    }
    
    Ok(json_files)
}

pub fn get_folders_in_folder(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut folders = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(folder_name) = path.file_name().and_then(|s| s.to_str()) {
                folders.push(folder_name.to_string());
            }
        }
    }

    Ok(folders)
}

pub fn load_json<T: serde::de::DeserializeOwned, P: AsRef<Path>>(path: P) -> Result<T, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(&path)?;
    let parsed = serde_json::from_str(&data)?;
    Ok(parsed)
}
