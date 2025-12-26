mod app;
mod profile;
mod storage;

use app::ProfileApp;
use clap::Parser;
use eframe::egui;

#[derive(Parser, Debug)]
#[command(name = "simple-profiles-manager")]
#[command(about = "A simple profile manager for applications")]
struct Args {
    /// Application ID (used for storage directory)
    #[arg(short, long)]
    app_id: String,

    /// Application title (displayed in the UI, defaults to app_id)
    #[arg(short, long)]
    title: Option<String>,

    /// Environment variable name to set with the selected profile name
    #[arg(short, long)]
    env_var: Option<String>,

    /// Program to execute after profile selection (full path)
    #[arg(short, long)]
    program: Option<String>,
}

fn main() -> eframe::Result<()> {
    let args = Args::parse();
    let app_title = args.title.unwrap_or_else(|| args.app_id.clone());

    storage::set_app_id(&args.app_id);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 350.0])
            .with_min_inner_size([300.0, 250.0]),
        ..Default::default()
    };

    eframe::run_native(
        &format!("Profile Manager - {}", app_title),
        options,
        Box::new(move |_cc| {
            Ok(Box::new(ProfileApp::new(
                app_title.clone(),
                args.env_var.clone(),
                args.program.clone(),
            )))
        }),
    )
}
