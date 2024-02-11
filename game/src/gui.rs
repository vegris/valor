use egui::{Context, FullOutput, RawInput, Ui};

use crate::{graphics::statics::Buttons, input::FrameInput};

pub fn create_context() -> Context {
    Context::default()
}

pub fn create_frame(ctx: &Context, input: &FrameInput) -> FullOutput {
    let raw_input = raw_input_from_frame_input(input);

    ctx.run(raw_input, |ctx| {
        egui::Area::new("menu")
            .fixed_pos((1., 555.))
            .show(ctx, menu);
    })
}

fn menu(ui: &mut Ui) {
    let buttons = [
        (Buttons::Settings, 4),
        (Buttons::Surrender, 55),
        (Buttons::Retreat, 106),
        (Buttons::AutoBattle, 157),
        (Buttons::BookOfMagic, 646),
        (Buttons::Wait, 697),
        (Buttons::Defend, 748),
    ];

    for (b, x) in buttons.into_iter() {
        let name: &'static str = b.into();
        let button = egui::Button::image(name);
        let rect = egui::Rect::from_two_pos(
            (x as f32, 555.).into(),
            ((x + 47) as f32, 555. + 44.).into(),
        );
        if ui.put(rect, button).clicked() {
            dbg!(name);
        }
    }
}

fn raw_input_from_frame_input(frame_input: &FrameInput) -> RawInput {
    let mut raw_input = RawInput::default();

    let cursor_pos = egui::pos2(
        frame_input.cursor_position.0 as f32,
        frame_input.cursor_position.1 as f32,
    );

    if frame_input.btn_lmb {
        raw_input.events.push(egui::Event::PointerButton {
            pos: cursor_pos,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        });
        raw_input.events.push(egui::Event::PointerButton {
            pos: cursor_pos,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        });
    }

    raw_input
}
