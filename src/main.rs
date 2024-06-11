#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod utils;
mod host;
use host::{Host, HonestHost, EvilHost};
use eframe::egui;
use regex::Regex;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([560.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Zk-mastermind",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp<HonestHost>>::default()
            //Box::<MyApp<EvilHost>>::default()
        }),
    )
}

// #[derive(Default, Copy, Clone)]
// struct Sequence {
//     seq: [char; 4],
// }
//
// impl Sequence {
//     fn to_string(&self) -> String {
//         self.seq.iter().collect()
//     }
// }
//
// #[derive(Default, Copy, Clone)]
// struct Response {
//     seq: [char; 4],
// }
//
// impl Response {
//     fn to_string(&self) -> String {
//         self.seq.iter().collect()
//     }
// }

const GUESSES: usize = 8;

// #[derive(Default)]
struct MyApp <H> {
    host: H,
    // guesses: [Sequence; GUESSES],
    // responses: [Sequence; GUESSES],
    responses: Vec<String>,
    guesses_made: u32,
    buffer: Vec<String>,
}

// TODO: do we need all of this?
impl <H> Default for MyApp<H> where H: Host {
    fn default() -> Self {
        Self {
            // seq: rand::thread_rng().sample_iter(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']).take(4).map(char::from).collect(),
            // TODO: make this random
            host: H::new(),
            // guesses: [Sequence::default(); GUESSES],
            // responses: [Sequence::default(); GUESSES],
            responses: vec![String::new(); GUESSES],
            guesses_made: 0,
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
            let mut response = ['x'; 4];
            for j in 0usize..common {
                response[j] = 'a';
            }
            for j in 0usize..same {
                response[j] = 'A';
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
                        ui.add_enabled_ui(((self.guesses_made as usize) == $i), |ui| {
                            let name_label = ui.label(format!("Guess {}: ", ($i)+1));
                            // TODO: make this narrower
                            ui.text_edit_singleline(&mut self.buffer[$i])
                                .labelled_by(name_label.id);
                            if ui.button("Confirm").clicked() {
                                self.responses[$i] = self.submit($i);
                                if !self.responses[$i].is_empty() {
                                    self.guesses_made += 1;
                                }
                            }
                        });
                        ui.label(format!("{}", self.responses[$i].to_string()));
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
        });
    }
}
