use anyhow::Result;
use clap::Parser;
use eframe::egui;
use kmobile_desktop::{Args, KMobileDesktopApp};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(if args.debug {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("🚀 Starting KMobile Desktop - Hardware Emulation & Visual Control");
    info!("🎯 Revolutionary mobile device control with full hardware simulation");

    // Initialize the desktop application
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(load_icon())
            .with_fullscreen(args.fullscreen),
        ..Default::default()
    };

    let app = KMobileDesktopApp::new(&args).await?;

    eframe::run_native(
        "KMobile Desktop - Hardware Emulation Control",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run desktop app: {}", e))
}

fn load_icon() -> egui::IconData {
    // Create a simple icon for now
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/kmobile-icon.png");
        let image = image::load_from_memory(icon)
            .unwrap_or_else(|_| {
                // Fallback: create a simple colored square
                image::ImageBuffer::from_fn(64, 64, |x, y| {
                    if (x + y) % 8 < 4 {
                        image::Rgba([0u8, 150u8, 255u8, 255u8]) // KMobile blue
                    } else {
                        image::Rgba([255u8, 255u8, 255u8, 255u8]) // White
                    }
                })
                .into()
            })
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
