#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod utils;
mod host;
mod consts;

use host::{Host, HonestHost};
use eframe::egui;
use regex::Regex;

const GUESSES: usize = 8;
const SEQLEN: usize = 4;


fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Zk-mastermind",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp<HonestHost>>::default()
        }),
    )
}

// #[derive(Default)]
struct MyApp <H> {
    host: H,
    responses: Vec<String>,
    guesses_cnt: usize,
    buffer: Vec<String>,
}

// TODO: do we need all of this?
impl <H> Default for MyApp<H> where H: Host {
    fn default() -> Self {
        Self {
            host: H::new(),
            responses: vec![String::new(); GUESSES],
            guesses_cnt: 0,
            buffer: vec![String::new(); GUESSES],
        }
    }
}

impl <H>  MyApp <H> where H:Host {
    fn submit(&mut self, i: usize) -> String {
        // TODO: make this global
        let pattern: Regex = Regex::new(r"^[a-h]{4}$").unwrap();
        let mut s = self.buffer[i].clone();
        if pattern.is_match(&mut s) {
            let (same, common) = self.host.guess(s);
            let mut response = ['x'; SEQLEN];
            for j in 0usize..common {
                response[j] = 'y';
            }
            for j in 0usize..same {
                response[j] = 'z';
            }
            return response.iter().collect::<String>()
        } // otherwise, do nothing
        "".to_owned()
    }
}


impl <H> eframe::App for MyApp <H> where H:Host {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            macro_rules! new_row {
                ($i:literal) => {
                    ui.horizontal(|ui| {
                        ui.add_enabled_ui(((self.guesses_cnt) == $i), |ui| {
                            let name_label = ui.label(format!("Guess {}: ", ($i)+1));
                            let (response, painter) = ui.allocate_painter(
                                egui::Vec2::new(120.0, 30.0),
                                egui::Sense::hover(),
                            );
                            for j in 0..self.buffer[$i].len() {
                                let cx = 15.0 + 30.0*(j as f32);
                                painter.circle_filled(
                                    response.rect.min + egui::Vec2::new(cx, 15.0),
                                    10.0,
                                    consts::COLORS[&self.buffer[$i].chars().nth(j).unwrap()]
                                );
                            }

                            if ui.button("Confirm").clicked() {
                                self.responses[$i] = self.submit($i);
                                if !self.responses[$i].is_empty() {
                                    self.guesses_cnt += 1;
                                }
                            }
                        });
                        let (response2, painter2) = ui.allocate_painter(
                            egui::Vec2::new(120.0, 30.0),
                            egui::Sense::hover(),
                        );
                        for j in 0..self.responses[$i].len() {
                            let cx = 15.0 + 30.0*(j as f32);
                            painter2.circle_filled(
                                response2.rect.min + egui::Vec2::new(cx, 15.0),
                                10.0,
                                consts::COLORS[&self.responses[$i].chars().nth(j).unwrap()]
                            );
                        }
                    });
                }
            }
            // TODO: can we get rid of this?
            new_row!(0usize);
            new_row!(1usize);
            new_row!(2usize);
            new_row!(3usize);
            new_row!(4usize);
            new_row!(5usize);
            new_row!(6usize);
            new_row!(7usize);
            ui.add(
                egui::Image::new(egui::include_image!("../data/color_map.png")).max_width(200.0)
            );

            for (letter, color) in consts::COLORS.clone().into_iter() {
                let key = egui::Key::from_name(&letter.to_string()).unwrap();
                if ui.input(|u| u.key_pressed(key)) {
                    if self.buffer[self.guesses_cnt].len() < SEQLEN {
                        self.buffer[self.guesses_cnt].push(letter);
                    }
                }
            }
            if ui.input(|u| u.key_pressed(egui::Key::Backspace)) {
                self.buffer[self.guesses_cnt].pop();
            }
            if ui.input(|u| u.key_pressed(egui::Key::Enter)) {
                // TODO: get rid of code duplication
                self.responses[self.guesses_cnt] = self.submit(self.guesses_cnt);
                if !self.responses[self.guesses_cnt].is_empty() {
                    self.guesses_cnt += 1;
                }
            }
        });
    }
}

