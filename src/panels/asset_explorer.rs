use egui::ScrollArea;
use std::fs;
use bevy_editor_pls::{*, editor_window::{EditorWindow, EditorWindowContext}};
use bevy::prelude::*;

pub struct AssetExplorerState {
    current_dir: std::path::PathBuf,
    selected_label : Option<String>
}

impl Default for AssetExplorerState {
    fn default() -> Self {
        Self { 
            current_dir: std::env::current_dir().unwrap(),
            selected_label: None
        }
    }
}

pub struct AssetExplorer;

impl EditorWindow for AssetExplorer {
    type State = AssetExplorerState;
    const NAME: &'static str = "Asset Explorer";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<AssetExplorer>().unwrap();
    
        egui::TopBottomPanel::top("asset_top").show_inside(ui, |ui| {
            // TODO: Add top panel contents
        });
    
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("assets").show(ui, |ui| {
                    // Button to go up one directory level
                    if ui.button("Up").clicked() {
                        // Get the parent directory path
                        if let Some(parent) = state.current_dir.parent() {
                            state.current_dir = parent.to_path_buf();
                        }
                    }
                    ui.end_row();
    
                    // File/directory entries
                    if let Ok(entries) = fs::read_dir(&state.current_dir) {
                        for entry in entries {
                            if let Ok(entry) = entry {
                                if let Ok(file_type) = entry.file_type() {
                                    let label = if file_type.is_dir() {
                                        "🗀 "
                                    } else {
                                        "🗋 "
                                    }.to_string() + &entry.file_name().to_string_lossy().to_string();
    
                                    let selected = state.selected_label == Some(label.clone());
    
                                    let mut sel_label = ui.selectable_label(selected, &label);
                                    if sel_label.clicked() {
                                        state.selected_label = Some(label.clone());
                                    }
                                    if sel_label.double_clicked() {
                                        state.selected_label = None;
                                        state.current_dir = entry.path();
                                    }
    
                                    ui.end_row();
                                }
                            }
                        }
                    }
                });
            });
        });
    }
    
}

// impl epi::App for FileExplorer {
//     fn name(&self) -> &str {
//         "Asset Explorer"
//     }

//     fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {

//     }
// }

// #[tokio::main]
// async fn main() {
//     let app = FileExplorer {
//         current_dir: std::env::current_dir().unwrap(),
//     };
//     eframe::run_native(Box::new(app), eframe::NativeOptions::default());
// }
