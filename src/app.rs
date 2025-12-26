use eframe::egui::{self, Color32, FontId, RichText, Rounding, Stroke, Vec2};

use crate::profile::Profile;
use crate::storage;

#[derive(Debug, Clone, PartialEq)]
enum AppState {
    ProfileList,
    NewProfile,
    EditProfile(usize),
}

pub struct ProfileApp {
    app_title: String,
    profiles: Vec<Profile>,
    selected_index: Option<usize>,
    state: AppState,
    profile_name_input: String,
    profile_selected: bool,
    focus_input: bool,
}

impl ProfileApp {
    pub fn new(app_title: String) -> Self {
        let profiles = storage::load_profiles();
        let selected_name = storage::load_selected_profile();

        let selected_index = if profiles.is_empty() {
            None
        } else {
            selected_name
                .and_then(|name| profiles.iter().position(|p| p.name == name))
                .or(Some(profiles.len() - 1))
        };

        let state = if profiles.is_empty() {
            AppState::NewProfile
        } else {
            AppState::ProfileList
        };

        let focus_input = profiles.is_empty();

        Self {
            app_title,
            profiles,
            selected_index,
            state,
            profile_name_input: String::new(),
            profile_selected: false,
            focus_input,
        }
    }

    fn get_selected_profile(&self) -> Option<&Profile> {
        self.selected_index.and_then(|i| self.profiles.get(i))
    }

    fn save_profiles(&self) {
        storage::save_profiles(&self.profiles);
    }

    fn render_profile_list(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(
                RichText::new(&self.app_title)
                    .font(FontId::proportional(24.0))
                    .color(Color32::from_rgb(100, 180, 255)),
            );
            ui.add_space(5.0);
            ui.label(
                RichText::new("Select or create a profile")
                    .font(FontId::proportional(14.0))
                    .color(Color32::GRAY),
            );
        });

        ui.add_space(20.0);

        if self.profiles.is_empty() {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("No profiles found")
                        .font(FontId::proportional(16.0))
                        .color(Color32::from_rgb(255, 180, 100)),
                );
                ui.add_space(5.0);
                ui.label("Create a new profile to get started");
            });
        } else {
            egui::Frame::none()
                .fill(Color32::from_rgb(35, 35, 40))
                .rounding(Rounding::same(8.0))
                .inner_margin(10.0)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .max_height(180.0)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            for (i, profile) in self.profiles.iter().enumerate() {
                                let is_selected = self.selected_index == Some(i);
                                let bg_color = if is_selected {
                                    Color32::from_rgb(60, 100, 160)
                                } else {
                                    Color32::TRANSPARENT
                                };
                                let text_color = if is_selected {
                                    Color32::WHITE
                                } else {
                                    Color32::from_rgb(200, 200, 200)
                                };

                                let row_response = egui::Frame::none()
                                    .fill(bg_color)
                                    .rounding(Rounding::same(4.0))
                                    .inner_margin(Vec2::new(10.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.set_width(ui.available_width());
                                        ui.label(
                                            RichText::new(&profile.name)
                                                .font(FontId::proportional(15.0))
                                                .color(text_color),
                                        );
                                    });
                                if row_response.response.interact(egui::Sense::click()).clicked() {
                                    self.selected_index = Some(i);
                                }
                                ui.add_space(2.0);
                            }
                        });
                });
        }

        ui.add_space(20.0);

        let button_size = Vec2::new(80.0, 30.0);
        let has_selection = self.selected_index.is_some();
        let num_buttons = if has_selection { 3 } else { 1 };
        let total_width = (button_size.x * num_buttons as f32) + (8.0 * (num_buttons - 1) as f32);
        let available_width = ui.available_width();
        let offset = (available_width - total_width) / 2.0;

        ui.horizontal(|ui| {
            ui.add_space(offset);

            if styled_button(ui, "New", Color32::from_rgb(80, 160, 80), button_size).clicked() {
                self.profile_name_input.clear();
                self.focus_input = true;
                self.state = AppState::NewProfile;
            }

            if let Some(idx) = self.selected_index {
                if styled_button(ui, "Edit", Color32::from_rgb(80, 130, 180), button_size).clicked()
                {
                    self.profile_name_input = self.profiles[idx].name.clone();
                    self.focus_input = true;
                    self.state = AppState::EditProfile(idx);
                }

                if styled_button(ui, "Delete", Color32::from_rgb(180, 80, 80), button_size).clicked()
                {
                    self.profiles.remove(idx);
                    self.save_profiles();

                    if self.profiles.is_empty() {
                        self.selected_index = None;
                        self.focus_input = true;
                        self.state = AppState::NewProfile;
                    } else {
                        self.selected_index =
                            Some(idx.saturating_sub(1).min(self.profiles.len() - 1));
                    }
                }
            }
        });

        ui.add_space(20.0);

        if self.selected_index.is_some() {
            ui.vertical_centered(|ui| {
                if styled_button(
                    ui,
                    "Select Profile",
                    Color32::from_rgb(100, 180, 100),
                    Vec2::new(150.0, 40.0),
                )
                .clicked()
                {
                    if let Some(profile) = self.get_selected_profile() {
                        storage::save_selected_profile(&profile.name);
                        self.profile_selected = true;
                    }
                }
            });
        }
    }

    fn render_new_profile(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(
                RichText::new("New Profile")
                    .font(FontId::proportional(24.0))
                    .color(Color32::from_rgb(100, 200, 100)),
            );
            ui.add_space(5.0);
            ui.label(
                RichText::new("Enter a name for your new profile")
                    .font(FontId::proportional(14.0))
                    .color(Color32::GRAY),
            );
        });

        ui.add_space(30.0);

        egui::Frame::none()
            .fill(Color32::from_rgb(35, 35, 40))
            .rounding(Rounding::same(8.0))
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Name:")
                            .font(FontId::proportional(15.0))
                            .color(Color32::from_rgb(180, 180, 180)),
                    );
                    ui.add_space(10.0);
                    let text_edit = egui::TextEdit::singleline(&mut self.profile_name_input)
                        .font(FontId::proportional(15.0))
                        .desired_width(200.0);
                    let response = ui.add(text_edit);
                    if self.focus_input {
                        response.request_focus();
                        self.focus_input = false;
                    }
                });
            });

        ui.add_space(20.0);

        let name_valid = !self.profile_name_input.trim().is_empty();
        let name_exists = self
            .profiles
            .iter()
            .any(|p| p.name == self.profile_name_input.trim());

        if name_exists {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("A profile with this name already exists")
                        .color(Color32::from_rgb(255, 100, 100)),
                );
            });
            ui.add_space(10.0);
        }

        ui.add_space(10.0);

        let button_size = Vec2::new(100.0, 35.0);
        let has_cancel = !self.profiles.is_empty();
        let num_buttons = if has_cancel { 2 } else { 1 };
        let total_width = (button_size.x * num_buttons as f32) + (10.0 * (num_buttons - 1) as f32);
        let available_width = ui.available_width();
        let offset = (available_width - total_width) / 2.0;

        let create_enabled = name_valid && !name_exists;
        let create_color = if create_enabled {
            Color32::from_rgb(80, 160, 80)
        } else {
            Color32::from_rgb(60, 60, 60)
        };

        ui.horizontal(|ui| {
            ui.add_space(offset);

            if has_cancel {
                if styled_button(ui, "Cancel", Color32::from_rgb(100, 100, 100), button_size)
                    .clicked()
                {
                    self.profile_name_input.clear();
                    self.state = AppState::ProfileList;
                }

                ui.add_space(10.0);
            }

            if ui
                .add_enabled(
                    create_enabled,
                    egui::Button::new(
                        RichText::new("Create")
                            .font(FontId::proportional(14.0))
                            .color(Color32::WHITE),
                    )
                    .fill(create_color)
                    .rounding(Rounding::same(6.0))
                    .min_size(button_size),
                )
                .clicked()
            {
                let profile = Profile::new(self.profile_name_input.trim().to_string());
                self.profiles.push(profile);
                self.save_profiles();
                self.selected_index = Some(self.profiles.len() - 1);
                self.profile_name_input.clear();
                self.state = AppState::ProfileList;
            }
        });
    }

    fn render_edit_profile(&mut self, ui: &mut egui::Ui, edit_index: usize) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.label(
                RichText::new("Edit Profile")
                    .font(FontId::proportional(24.0))
                    .color(Color32::from_rgb(100, 150, 255)),
            );
            ui.add_space(5.0);
            ui.label(
                RichText::new("Modify the profile name")
                    .font(FontId::proportional(14.0))
                    .color(Color32::GRAY),
            );
        });

        ui.add_space(30.0);

        egui::Frame::none()
            .fill(Color32::from_rgb(35, 35, 40))
            .rounding(Rounding::same(8.0))
            .inner_margin(20.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Name:")
                            .font(FontId::proportional(15.0))
                            .color(Color32::from_rgb(180, 180, 180)),
                    );
                    ui.add_space(10.0);
                    let text_edit = egui::TextEdit::singleline(&mut self.profile_name_input)
                        .font(FontId::proportional(15.0))
                        .desired_width(200.0);
                    let response = ui.add(text_edit);
                    if self.focus_input {
                        response.request_focus();
                        self.focus_input = false;
                    }
                });
            });

        ui.add_space(20.0);

        let name_valid = !self.profile_name_input.trim().is_empty();
        let original_name = &self.profiles[edit_index].name;
        let name_changed = self.profile_name_input.trim() != original_name;
        let name_exists = name_changed
            && self
                .profiles
                .iter()
                .any(|p| p.name == self.profile_name_input.trim());

        if name_exists {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new("A profile with this name already exists")
                        .color(Color32::from_rgb(255, 100, 100)),
                );
            });
            ui.add_space(10.0);
        }

        ui.add_space(10.0);

        let button_size = Vec2::new(100.0, 35.0);
        let total_width = (button_size.x * 2.0) + 10.0;
        let available_width = ui.available_width();
        let offset = (available_width - total_width) / 2.0;

        let save_enabled = name_valid && !name_exists;
        let save_color = if save_enabled {
            Color32::from_rgb(80, 130, 180)
        } else {
            Color32::from_rgb(60, 60, 60)
        };

        ui.horizontal(|ui| {
            ui.add_space(offset);

            if styled_button(ui, "Cancel", Color32::from_rgb(100, 100, 100), button_size).clicked()
            {
                self.profile_name_input.clear();
                self.state = AppState::ProfileList;
            }

            ui.add_space(10.0);

            if ui
                .add_enabled(
                    save_enabled,
                    egui::Button::new(
                        RichText::new("Save")
                            .font(FontId::proportional(14.0))
                            .color(Color32::WHITE),
                    )
                    .fill(save_color)
                    .rounding(Rounding::same(6.0))
                    .min_size(button_size),
                )
                .clicked()
            {
                self.profiles[edit_index].name = self.profile_name_input.trim().to_string();
                self.save_profiles();
                self.profile_name_input.clear();
                self.state = AppState::ProfileList;
            }
        });
    }
}

fn styled_button(ui: &mut egui::Ui, text: &str, color: Color32, size: Vec2) -> egui::Response {
    ui.add(
        egui::Button::new(
            RichText::new(text)
                .font(FontId::proportional(14.0))
                .color(Color32::WHITE),
        )
        .fill(color)
        .stroke(Stroke::new(1.0, color.linear_multiply(1.2)))
        .rounding(Rounding::same(6.0))
        .min_size(size),
    )
}

impl eframe::App for ProfileApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        if self.profile_selected {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.label(
                        RichText::new("Profile Selected!")
                            .font(FontId::proportional(28.0))
                            .color(Color32::from_rgb(100, 200, 100)),
                    );
                    ui.add_space(20.0);
                    if let Some(profile) = self.get_selected_profile() {
                        egui::Frame::none()
                            .fill(Color32::from_rgb(35, 35, 40))
                            .rounding(Rounding::same(8.0))
                            .inner_margin(20.0)
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new(&profile.name)
                                        .font(FontId::proportional(20.0))
                                        .color(Color32::WHITE),
                                );
                            });
                    }
                    ui.add_space(30.0);
                    ui.label(
                        RichText::new("You can now close this window")
                            .font(FontId::proportional(14.0))
                            .color(Color32::GRAY),
                    );
                });
            });
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| match self.state.clone() {
            AppState::ProfileList => self.render_profile_list(ui),
            AppState::NewProfile => self.render_new_profile(ui),
            AppState::EditProfile(idx) => self.render_edit_profile(ui, idx),
        });
    }
}
