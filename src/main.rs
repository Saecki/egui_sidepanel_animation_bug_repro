use egui::containers::panel::Side;
use egui::{CentralPanel, Frame, SidePanel};

fn main() {
    eframe::run_native(
        "egui_sidepanel_animation_bug_repro",
        Default::default(),
        Box::new(|c| {
            let mut style = c.egui_ctx.style().as_ref().clone();
            // slow down animation time to make the issue more apparent
            style.animation_time = 2.0;
            c.egui_ctx.set_style(style);
            Box::new(ReproApp::default())
        }),
    )
    .unwrap();
}

#[derive(Default)]
struct ReproApp {
    show_left: bool,
    show_right: bool,
}

impl eframe::App for ReproApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::new(Side::Left, "left")
            .frame(Frame::none().inner_margin(32.0).outer_margin(32.0))
            .show_animated(ctx, self.show_left, |ui| {
                ui.label("hi there");
            });
        
        SidePanel::new(Side::Right, "right")
            .frame(Frame::none().inner_margin(32.0).outer_margin(32.0))
            .show_animated(ctx, self.show_right, |ui| {
                ui.label("hi there");
            });

        CentralPanel::default().show(ctx, |ui| {
            if ui.button("toggle left").clicked() {
                self.show_left = !self.show_left;
            }
            if ui.button("toggle right").clicked() {
                self.show_right = !self.show_right;
            }
        });
    }
}
