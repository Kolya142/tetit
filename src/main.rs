mod shash;

use std::{fs::{self, rename}, time::{SystemTime, UNIX_EPOCH}};
use eframe::egui;
use egui::{menu, Ui};
use shash::shash_get;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "TETIT",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

fn random_name() -> String {
    let t = SystemTime::now();
    let since_the_epoch = t.duration_since(UNIX_EPOCH);
    let t = since_the_epoch.unwrap().as_micros();
    let h = shash_get(t as i128);
    let o = format!("file-{:x}.txt", h&0xffffff);
    o
}

struct MyApp {
    text: Vec<String>,
    filename: Vec<String>,
    fileindex: usize,
    is_menu: bool,
    menu_file_dialog: bool,
    menu_file_text: String
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            text: vec![String::new()],
            filename: vec![random_name()],
            fileindex: 0,
            is_menu: false,
            menu_file_dialog: false,
            menu_file_text: "file".to_string()
        }
    }
}

fn menu_default(s: &mut MyApp, ui: &mut Ui) {
    ui.horizontal(
        |ui| 
        {
            for i in 0..s.text.len() {
                if ui.button(format!("{}", i)).clicked() {
                    s.fileindex = i;
                }
            }
        }
    );
    if s.filename.len() > 0 {
        ui.horizontal(
            |ui| 
            {
                ui.label("remove one");
                for i in 0..s.text.len() {
                    if ui.button(format!("{}", i)).clicked() {
                        s.filename.remove(i);
                        s.text.remove(i);
                        s.fileindex -= 1;
                        break;
                    }
                }
            }
        );
    }
    if ui.button("create new").clicked() {
        s.fileindex+=1;
        s.filename.push(random_name());
        s.text.push("".to_string());
    }
    if ui.button("open menu").clicked() {
        s.is_menu = true;
    }
    ui.add_space(5f32);
    ui.separator();
    ui.add_space(5f32);
    let before_fn = format!("files/{}", s.filename[s.fileindex].clone());
    if ui.text_edit_singleline(&mut s.filename[s.fileindex]).changed() {
        let _ = rename(before_fn, format!("files/{}", s.filename[s.fileindex].clone()));
    }
    if ui.text_edit_multiline(&mut s.text[s.fileindex]).changed() {
        let _ = fs::write(format!("files/{}", s.filename[s.fileindex].clone()), s.text[s.fileindex].clone());
    }
}
fn menu_menu(s: &mut MyApp, ui: &mut Ui) {
    if ui.button("close menu").clicked() {
        s.is_menu = false;
    }
    if !s.menu_file_dialog {
        if ui.button("open file choice dialog").clicked() {
            s.menu_file_dialog = true;
        }
    }
    else {
        if ui.button("close file choice dialog").clicked() {
            s.menu_file_dialog = false;
        }
        ui.add_space(30f32);
        ui.horizontal(|ui| {
            for filename in fs::read_dir("files").unwrap() {
                let filename = filename.unwrap().file_name().into_string().unwrap();
                if ui.button(filename.clone()).clicked() {
                    s.menu_file_text = filename;
                }
            }
        });
        ui.text_edit_singleline(&mut s.menu_file_text);
        if ui.button("open it").clicked() {
            let m = fs::read_to_string(format!("files/{}", s.menu_file_text.clone()));
            if m.is_ok() {
                s.filename.push(s.menu_file_text.clone());
                let m = m.unwrap();
                s.text.push(m);
                s.fileindex += 1;
            }
        }
    }

}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TETIT Editor");
            if !self.is_menu {
                menu_default(self, ui);
            }
            else {
                menu_menu(self, ui);
            }
        });
    }
}
