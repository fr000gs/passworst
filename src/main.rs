#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
/*
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Call this once from the HTML.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start(canvas_id: &str) -> Result<AppRunnerRef, eframe::wasm_bindgen::JsValue> {
    let web_options = eframe::WebOptions::default();
    eframe::start_web(canvas_id, web_options, Box::new(|cc| Box::new(MyEguiApp::new(cc)))).await
}
*/
//#[cfg(target_os = "android")]
//#[no_mangle]
extern crate sha2;
extern crate base16ct;

use sha2::{Sha512, Digest};

use eframe::egui;

fn hash512(string: String) -> String {
    let mut sha512 = Sha512::new();
    sha512.update(string);
    let hash = sha512.finalize();
    base16ct::lower::encode_string(&hash)
}

fn truncate(hash: String) -> String {
    let mut value = String::new();
    let mut j = 0;
    for i in hash.chars() {
        if j % 8 == 0 {value += &i.to_string();}
        j += 1
    }
    value
}

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    //tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(420.0, 420.0)),
        min_window_size: Some(egui::vec2(420.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Password Manager",
        options,
        Box::new(|_cc| Box::new(Pswd::default())),
    )
}

struct Pswd {
    user: String,
    pswd: String,
    hash: String,
    display_user: bool,
    display_pswd: bool,
    display_hash: bool,
}

impl Default for Pswd {
    fn default() -> Self {
        Self {
            user: String::from(""),
            pswd: String::from(""),
            hash: String::from(""),
            display_user: false,
            display_pswd: false,
            display_hash: false,
        }
    }
}

impl eframe::App for Pswd {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Password Manager");
            ui.horizontal(|ui| {
                let name_label = ui.label("Username: ");
                ui.add(egui::TextEdit::singleline(&mut self.user)
                    .password(!self.display_user))
                    .labelled_by(name_label.id);
                ui.toggle_value(&mut self.display_user, "👁️");
            });
            ui.horizontal(|ui| {
                let pass_label = ui.label("Password: ");
                ui.add(egui::TextEdit::singleline(&mut self.pswd)
                    .password(!self.display_pswd))
                    .labelled_by(pass_label.id);                 
                ui.toggle_value(&mut self.display_pswd, "👁️");
            });
            ui.horizontal(|ui| {
                let hash_label = ui.label("Hash: ");
                self.hash =  truncate(hash512(self.pswd.clone() + &self.user))+"@A";
                ui.add(egui::TextEdit::singleline(&mut self.hash)
                    .password(!self.display_hash))
                    .labelled_by(hash_label.id);                 
                ui.toggle_value(&mut self.display_hash, "👁️");
            });
            if ui.button("Copy hash").clicked() {
                ui.output_mut(|o| o.copied_text = self.hash.clone());
                println!("{:?}", self.hash.clone());
            }
        });
    }
}
