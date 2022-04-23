use eframe::egui;
use egui::ScrollArea;

use crate::egs::EpicPlatform;

use super::{
    ui_colors::{BACKGROUND_COLOR, EXTRA_BACKGROUND_COLOR},
    MyEguiApp,
};
const SECTION_SPACING: f32 = 25.0;

impl MyEguiApp {
    pub(crate) fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");

        let mut scroll_style = ui.style_mut();
        scroll_style.visuals.extreme_bg_color = BACKGROUND_COLOR;
        scroll_style.visuals.widgets.inactive.bg_fill = EXTRA_BACKGROUND_COLOR;
        scroll_style.visuals.widgets.active.bg_fill = EXTRA_BACKGROUND_COLOR;
        scroll_style.visuals.selection.bg_fill = EXTRA_BACKGROUND_COLOR;
        scroll_style.visuals.widgets.hovered.bg_fill = EXTRA_BACKGROUND_COLOR;

        ScrollArea::vertical()
            .stick_to_right()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.reset_style();

                self.render_steamgriddb_settings(ui);

                self.render_steam_settings(ui);

                self.render_epic_settings(ui);

                #[cfg(target_family = "unix")]
                {
                    ui.heading("Heroic");
                    ui.checkbox(&mut self.settings.heroic.enabled, "Import form Heroic");

                    ui.add_space(SECTION_SPACING);
                }

                self.render_legendary_settings(ui);
                self.render_itch_settings(ui);
                self.render_origin_settings(ui);
                self.render_gog_settings(ui);
                self.render_uplay_settings(ui);
                self.render_lutris_settings(ui);
                #[cfg(windows)]
                {
                    self.render_amazon_settings(ui);
                }
            });
    }

    fn render_lutris_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Lutris");
        ui.checkbox(&mut self.settings.lutris.enabled, "Import form Lutris");
        if self.settings.lutris.enabled {
            ui.horizontal(|ui| {
                let mut empty_string = "".to_string();
                let lutris_location = self
                    .settings
                    .lutris
                    .executable
                    .as_mut()
                    .unwrap_or(&mut empty_string);
                ui.label("Lutris Location: ");
                if ui.text_edit_singleline(lutris_location).changed() {
                    self.settings.lutris.executable = Some(lutris_location.to_string());
                }
            });
        }
        ui.add_space(SECTION_SPACING);
    }

    #[cfg(windows)]
    fn render_amazon_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Amazon");
        ui.checkbox(&mut self.settings.amazon.enabled, "Import form Amazon");
        if self.settings.amazon.enabled {
            ui.checkbox(&mut self.settings.amazon.prepend, "Prepend 'Amazon Games.exe'");
            ui.horizontal(|ui| {
                let mut empty_string = "".to_string();
                let amazon_location = self
                    .settings
                    .amazon
                    .location
                    .as_mut()
                    .unwrap_or(&mut empty_string);
                ui.label("Amazon Games Folder: ");
                if ui.text_edit_singleline(amazon_location).changed() {
                    self.settings.amazon.location = Some(amazon_location.to_string());
                }
            });
        }
        ui.add_space(SECTION_SPACING);
    }

    fn render_uplay_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Uplay");
        ui.checkbox(&mut self.settings.uplay.enabled, "Import form Uplay");
        if self.settings.uplay.enabled {
            ui.checkbox(&mut self.settings.uplay.prepend, "Prepend 'Ubisoft Connect.exe'");
            ui.horizontal(|ui| {
                let mut empty_string = "".to_string();
                let uplay_location = self
                    .settings
                    .uplay
                    .location
                    .as_mut()
                    .unwrap_or(&mut empty_string);
                ui.label("Uplay Folder: ");
                if ui.text_edit_singleline(uplay_location).changed() {
                    self.settings.uplay.location = Some(uplay_location.to_string());
                }
            });
        }
        ui.add_space(SECTION_SPACING);
    }

    fn render_gog_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("GoG Galaxy");
        ui.checkbox(&mut self.settings.gog.enabled, "Import form GoG Galaxy");
        if self.settings.gog.enabled {
            ui.horizontal(|ui| {
                let mut empty_string = "".to_string();
                let itch_location = self
                    .settings
                    .gog
                    .location
                    .as_mut()
                    .unwrap_or(&mut empty_string);
                ui.label("GoG Galaxy Folder: ");
                if ui.text_edit_singleline(itch_location).changed() {
                    self.settings.gog.location = Some(itch_location.to_string());
                }
            });
        }
        ui.add_space(SECTION_SPACING);
    }

    fn render_origin_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Origin");
        ui.checkbox(&mut self.settings.origin.enabled, "Import from Origin");
        if self.settings.origin.enabled {
            ui.checkbox(&mut self.settings.origin.prepend, "Prepend Origin.exe");
            ui.horizontal(|ui| {
                let mut empty_string = "".to_string();
                let origin_location = self
                    .settings
                    .origin
                    .location
                    .as_mut()
                    .unwrap_or(&mut empty_string);
                ui.label("Origin Folder: ");
                if ui.text_edit_singleline(origin_location).changed() {
                    self.settings.origin.location = Some(origin_location.to_string());
                }
            });
        }
        ui.add_space(SECTION_SPACING);
    }

    fn render_itch_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Itch.io");
        ui.checkbox(&mut self.settings.itch.enabled, "Import form Itch.io");
        if self.settings.itch.enabled {
            ui.horizontal(|ui| {
                let mut empty_string = "".to_string();
                let itch_location = self
                    .settings
                    .itch
                    .location
                    .as_mut()
                    .unwrap_or(&mut empty_string);
                ui.label("Itch.io Folder: ");
                if ui.text_edit_singleline(itch_location).changed() {
                    self.settings.itch.location = Some(itch_location.to_string());
                }
            });
        }
        ui.add_space(SECTION_SPACING);
    }

    fn render_steam_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Steam");
        ui.horizontal(|ui| {
            let mut empty_string = "".to_string();
            let steam_location = self
                .settings
                .steam
                .location
                .as_mut()
                .unwrap_or(&mut empty_string);
            ui.label("Steam Location: ");
            if ui.text_edit_singleline(steam_location).changed() {
                self.settings.steam.location = Some(steam_location.to_string());
            }
        });
        ui.checkbox(
            &mut self.settings.steam.create_collections,
            "Create collections",
        )
        .on_hover_text("Tries to create a games collection for each platform");
        ui.checkbox(&mut self.settings.steam.optimize_for_big_picture, "Optimize for big picture").on_hover_text("Set icons to be larger horizontal images, this looks nice in steam big picture mode, but a bit off in desktop mode");
        ui.checkbox(
            &mut self.settings.steam.stop_steam,
            "Stop Steam before import",
        )
        .on_hover_text("Stops Steam if it is running when import starts");
        ui.checkbox(
            &mut self.settings.steam.start_steam,
            "Start Steam after import",
        )
        .on_hover_text("Starts Steam is it is not running after the import");
        ui.add_space(SECTION_SPACING);
    }

    fn render_steamgriddb_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("SteamGridDB");
        ui.checkbox(&mut self.settings.steamgrid_db.enabled, "Download images");
        if self.settings.steamgrid_db.enabled {
            ui.horizontal(|ui| {
                let mut empty_string = "".to_string();
                let auth_key = self
                    .settings
                    .steamgrid_db
                    .auth_key
                    .as_mut()
                    .unwrap_or(&mut empty_string);
                ui.label("Authentication key: ");
                if ui.text_edit_singleline(auth_key).changed() {
                    self.settings.steamgrid_db.auth_key = Some(auth_key.to_string());
                }
            });
            ui.horizontal(|ui| {
                ui.label(
                    "To download images you need an API Key from SteamGridDB, you can find yours",
                );
                ui.hyperlink_to(
                    "here",
                    "https://www.steamgriddb.com/profile/preferences/api",
                )
            });
            ui.checkbox(&mut self.settings.steamgrid_db.prefer_animated, "Prefer animated images").on_hover_text("Prefer downloading animated images over static images (this can slow Steam down but looks neat)");
        }
        ui.add_space(SECTION_SPACING);
    }

    fn render_legendary_settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Legendary & Rare");
        ui.checkbox(
            &mut self.settings.legendary.enabled,
            "Import form Legendary & Rare",
        );
        if self.settings.legendary.enabled {
            ui.horizontal(|ui| {
                let mut empty_string = "".to_string();
                let legendary_location = self
                    .settings
                    .legendary
                    .executable
                    .as_mut()
                    .unwrap_or(&mut empty_string);
                ui.label("Legendary Executable: ")
                    .on_hover_text("The location of the legendary executable to use");
                if ui.text_edit_singleline(legendary_location).changed() {
                    self.settings.legendary.executable = Some(legendary_location.to_string());
                }
            });
        }
        ui.add_space(SECTION_SPACING);
    }

    fn render_epic_settings(&mut self, ui: &mut egui::Ui) {
        let epic_settings = &mut self.settings.epic_games;
        ui.heading("Epic Games");
        ui.checkbox(&mut epic_settings.enabled, "Import form Epic Games");
        if epic_settings.enabled {
            ui.horizontal(|ui| {
                let mut empty_string = "".to_string();
                let epic_location = epic_settings.location.as_mut().unwrap_or(&mut empty_string);
                ui.label("Epic Manifests Location: ").on_hover_text(
                    "The location where Epic stores its manifest files that BoilR needs to read",
                );
                if ui.text_edit_singleline(epic_location).changed() {
                    epic_settings.location = Some(epic_location.to_string());
                }
            });

            let safe_mode_header = match epic_settings.safe_launch.len() {
                0 => "Force games to launch through Epic Launcher".to_string(),
                1 => "One game forced to launch through Epic Launcher".to_string(),
                x => format!("{} games forced to launch through Epic Launcher", x),
            };

            egui::CollapsingHeader::new(safe_mode_header)
        .id_source("Epic_Launcher_safe_launch")
        .show(ui, |ui| {
            ui.label("Some games must be started from the Epic Launcher, select those games below and BoilR will create shortcuts that opens the games through the Epic Launcher.");
            let manifests =self.epic_manifests.get_or_insert_with(||{
                let epic_platform = EpicPlatform::new(epic_settings);
                let manifests = crate::platform::Platform::get_shortcuts(&epic_platform);
                manifests.unwrap_or_default()
            });
            let mut safe_open_games = epic_settings.safe_launch.clone();
            for manifest in manifests{
                let key = manifest.get_key();
                let display_name = &manifest.display_name;
                let mut safe_open = safe_open_games.contains(display_name) || safe_open_games.contains(&key);
                if ui.checkbox(&mut safe_open, display_name).clicked(){
                    if safe_open{
                        safe_open_games.push(key);
                    }else{
                        safe_open_games.retain(|m| m!= display_name && m!= &key);
                    }
                }
            }
            epic_settings.safe_launch = safe_open_games;
        })        ;
            ui.add_space(SECTION_SPACING);
        }
    }
}
