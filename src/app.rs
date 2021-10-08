use eframe::{egui, epi};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(featrue = "persistence", serde(default))]
pub struct App {
    label:String,

    #[cfg_attr(feature = "persistence", serde(skip))]
    value:f32
}

impl Default for App {
    fn default() -> Self {
        Self {
            label: "wasd".to_owned(),
            value: 3.4
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "EGUI CROSSPLATFORM TESTS"
    }

    fn setup(&mut self, _ctx:&egui::CtxRef, _frame:&mut epi::Frame<'_>, _storage:Option<&dyn epi::Storage>) {
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage:&mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx:&egui::CtxRef, frame:&mut epi::Frame<'_>) {
        let Self {label, value} = self;

        #[cfg(not(target_arch = "wasm32"))]
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("things on the side...");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::new("https://google.com/").text("not powered by google"));
            });
        });
    }
}