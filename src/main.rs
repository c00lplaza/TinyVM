use std::io;

mod cpu;
mod memory;
mod disk;
mod iso;
mod gui;
mod net;
mod sound;
mod graphics;
mod input;
mod snapshot;
mod plugin;
mod scripting;
mod bios;
mod elf_loader;
mod fat32;
mod jit;
mod utils;
mod easter_egg;
mod games;
mod meme_mode;
mod api;
mod cloud;

fn main() {
    println!("Welcome to TinyVM!");
    // Launch GUI
    gui::run();
}

use eframe::{egui, epi};

struct TinyVMApp {
    status: String,
}

impl Default for TinyVMApp {
    fn default() -> Self {
        Self {
            status: "Ready".to_owned(),
        }
    }
}

impl epi::App for TinyVMApp {
    fn name(&self) -> &str {
        "TinyVM GUI"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TinyVM");
            if ui.button("Load ISO").clicked() {
                // TODO: Add file picker and ISO loading logic
                self.status = "ISO Loaded!".to_owned();
            }
            ui.label(format!("Status: {}", self.status));
        });
    }
}

struct TinyVM {
    stack: Vec<i32>,
}

impl TinyVM {
    fn new() -> Self {
        TinyVM { stack: Vec::new() }
    }

    fn run(&mut self, program: &[&str]) {
        for instr in program {
            let parts: Vec<&str> = instr.split_whitespace().collect();
            match parts[0] {
                "PUSH" => {
                    let n: i32 = parts[1].parse().expect("Invalid number for PUSH");
                    self.stack.push(n);
                }
                "ADD" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                "SUB" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                "MUL" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                "DIV" => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }
                "DUP" => {
                    let a = *self.stack.last().unwrap();
                    self.stack.push(a);
                }
                "SWAP" => {
                    let len = self.stack.len();
                    self.stack.swap(len - 1, len - 2);
                }
                "PRINT" => {
                    println!("{}", self.stack.pop().unwrap());
                }
                _ => panic!("Unknown instruction: {}", instr),
            }
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "TinyVM GUI",
        options,
        Box::new(|_cc| Box::new(TinyVMApp::default())),
    )
}