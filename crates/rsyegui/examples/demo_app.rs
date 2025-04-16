use std::time::Duration;

use eframe;
use egui::{Color32, Shadow, Slider, Visuals, Window};
use rsyegui::{
    dropdown::dropdown_box::DropDownBox,
    notify::{Toast, Toasts},
};
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            title: Some("RsyEgui Demo App".to_owned()),
            inner_size: Some([800., 600.].into()),
            ..Default::default()
        },
        ..Default::default()
    };
    eframe::run_native(
        "RsyEgui Demo App",
        options,
        Box::new(|_cc| Ok(Box::new(RsyEguiApp::new()))),
    )
}

pub struct RsyEguiApp {
    drop_down_box_app: DropDownBoxApp,
    notify_app: NotifyApp,
}
impl RsyEguiApp {
    pub fn new() -> Self {
        let items = vec![
            "Item 1".to_string(),
            "Item 2".to_string(),
            "Item 3".to_string(),
        ];
        let buf = String::new();
        Self {
            drop_down_box_app: DropDownBoxApp::new(items, buf),
            notify_app: NotifyApp::new(),
        }
    }
}

impl eframe::App for RsyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            self.drop_down_box_app.update(ctx, frame);
            self.notify_app.update(ctx, frame);
        });
    }
}

struct DropDownBoxApp {
    items: Vec<String>,
    selected_item: Option<String>,
    buf: String,
}

impl DropDownBoxApp {
    fn new(items: Vec<String>, buf: String) -> Self {
        Self {
            items: items,
            selected_item: None,
            buf: buf,
        }
    }
}

pub trait AppView {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame);
}

impl AppView for DropDownBoxApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    DropDownBox::from_iter(
                        &self.items,
                        "test_dropbox",
                        &mut self.buf,
                        |ui, text| {
                            self.selected_item = Some(text.to_owned());
                            ui.selectable_label(false, text)
                        },
                    )
                    // choose whether to filter the box items based on what is in the text edit already
                    // default is true when this is not used
                    .filter_by_input(true)
                    // choose whether to select all text in the text edit when it gets focused
                    // default is false when this is not used
                    .select_on_focus(true)
                    // passes through the desired width to the text edit
                    // default is None internally, so TextEdit does whatever its default implements
                    .desired_width(250.0),
                );

                if ui.button("Add").clicked() {
                    if self.buf.is_empty() {
                        return;
                    }
                    self.items.push(self.buf.clone());
                }
            });
            println!("Selected item: {:?}", self.selected_item);
        });
    }
}

struct NotifyApp {
    toasts: Toasts,
    caption: String,
    closable: bool,
    show_progress_bar: bool,
    expires: bool,
    duration: f32,
    font_size: f32,
    dark: bool,
    custom_level_string: String,
    custom_level_color: Color32,
    shadow: bool,
}

impl NotifyApp {
    pub fn new() -> Self {
        Self {
            caption: r#"Hello! It's a multiline caption
Next line
Another one
And another one"#
                .into(),
            toasts: Toasts::default(),
            closable: true,
            expires: true,
            show_progress_bar: true,
            duration: 3.5,
            dark: true,
            font_size: 16.,
            custom_level_string: "$".into(),
            custom_level_color: egui::Color32::GREEN,
            shadow: true,
        }
    }
}

impl AppView for NotifyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        Window::new("Controls").show(ctx, |ui| {
            ui.text_edit_multiline(&mut self.caption);
            ui.checkbox(&mut self.expires, "Expires");
            ui.checkbox(&mut self.closable, "Closable");
            ui.checkbox(&mut self.show_progress_bar, "ShowProgressBar");
            ui.checkbox(&mut self.shadow, "Shadow").clicked().then(|| {
                self.toasts = if self.shadow {
                    Toasts::default().with_shadow(Shadow {
                        offset: Default::default(),
                        blur: 30,
                        spread: 5,
                        color: Color32::from_black_alpha(70),
                    })
                } else {
                    Toasts::default()
                };
            });
            if !(self.expires || self.closable) {
                ui.label("Warning; toasts will have to be closed programatically");
            }
            ui.add_enabled_ui(self.expires, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Duration (in s)");
                    ui.add(Slider::new(&mut self.duration, 1.0..=10.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Font size");
                    ui.add(Slider::new(&mut self.font_size, 8.0..=20.0));
                });
            });
            ui.text_edit_singleline(&mut self.custom_level_string);
            ui.color_edit_button_srgba(&mut self.custom_level_color);

            let customize_toast = |t: &mut Toast| {
                let duration = if self.expires {
                    Some(Duration::from_millis((1000. * self.duration) as u64))
                } else {
                    None
                };

                t.closable(self.closable)
                    .duration(duration)
                    .show_progress_bar(self.show_progress_bar);
            };

            ui.horizontal(|ui| {
                if ui.button("Success").clicked() {
                    customize_toast(self.toasts.success(self.caption.clone()));
                }

                if ui.button("Info").clicked() {
                    customize_toast(self.toasts.info(self.caption.clone()));
                }

                if ui.button("Warning").clicked() {
                    customize_toast(self.toasts.warning(self.caption.clone()));
                }

                if ui.button("Error").clicked() {
                    customize_toast(self.toasts.error(self.caption.clone()));
                }

                if ui.button("Basic").clicked() {
                    customize_toast(self.toasts.basic(self.caption.clone()));
                }

                if ui.button("Rich text").clicked() {
                    customize_toast(
                        self.toasts.success(
                            egui::RichText::new(self.caption.clone())
                                .color(Color32::GREEN)
                                .background_color(Color32::DARK_GRAY)
                                .size(self.font_size)
                                .italics()
                                .underline(),
                        ),
                    );
                }

                if ui.button("Custom").clicked() {
                    customize_toast(self.toasts.custom(
                        self.caption.clone(),
                        self.custom_level_string.clone(),
                        self.custom_level_color,
                    ));
                }

                if ui
                    .button("Phosphor")
                    .on_hover_text("This toast uses egui-phosphor icons")
                    .clicked()
                {
                    // customize_toast(self.toasts.custom(
                    //     self.caption.clone(),
                    //     egui_phosphor::regular::FAN.to_owned(),
                    //     self.custom_level_color,
                    // ));
                }
            });

            ui.separator();

            if ui.button("Dismiss all toasts").clicked() {
                self.toasts.dismiss_all_toasts();
            }
            if ui.button("Dismiss latest toast").clicked() {
                self.toasts.dismiss_latest_toast();
            }
            if ui.button("Dismiss oldest toast").clicked() {
                self.toasts.dismiss_oldest_toast();
            }

            ui.separator();

            if ui.radio(self.dark, "Toggle dark theme").clicked() {
                self.dark = !self.dark;

                let mut style = ctx.style().as_ref().clone();
                if self.dark {
                    style.visuals = Visuals::dark();
                } else {
                    style.visuals = Visuals::light();
                }
                ctx.set_style(style);
            }
        });

        self.toasts.show(ctx);
    }
}
