use hash_forge::gui_core::HashForgeApp;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Hash Forge",
        native_options,
        Box::new(|_cc| Ok(Box::new(HashForgeApp::new()))),
    )
}

fn load_icon() -> egui::IconData {
    // Create a simple hash-themed icon: a grid pattern
    let width = 32;
    let height = 32;
    let mut pixels = vec![0u8; width * height * 4]; // RGBA

    // Draw a simple hash pattern
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;

            // Create a hash/grid pattern
            let is_grid = (x % 8 == 0 || x % 8 == 1) || (y % 8 == 0 || y % 8 == 1);
            let is_cross = (10..=13).contains(&x)
                || (10..=13).contains(&y)
                || (18..=21).contains(&x)
                || (18..=21).contains(&y);

            if is_grid || is_cross {
                pixels[idx] = 100; // R
                pixels[idx + 1] = 150; // G
                pixels[idx + 2] = 200; // B
                pixels[idx + 3] = 255; // A
            } else {
                pixels[idx] = 30; // R
                pixels[idx + 1] = 30; // G
                pixels[idx + 2] = 50; // B
                pixels[idx + 3] = 255; // A
            }
        }
    }

    egui::IconData {
        rgba: pixels,
        width: width as u32,
        height: height as u32,
    }
}
