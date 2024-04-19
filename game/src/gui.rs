pub mod textures;

use egui::{
    epaint::{ClippedShape, RectShape},
    Context, FullOutput, RawInput, Rect, Shape, TextureId, Ui,
};
use gamedata::spells::Spell;

use crate::{graphics::statics::Buttons, input::FrameInput, Stage};
use logic::command::Cast;

use self::textures::Button;

pub fn create_frame(
    ctx: &Context,
    input: &FrameInput,
    stage: &mut Stage,
    cast: &mut Option<Cast>,
) -> FullOutput {
    let raw_input = raw_input_from_frame_input(input);

    ctx.run(raw_input, |ctx| {
        egui::Area::new("menu")
            .fixed_pos((0., 556.))
            .show(ctx, |ui| menu(ui, stage));

        if matches!(stage, Stage::SpellBook) {
            egui::Area::new("spellbook")
                .fixed_pos((400. - 310., 0.))
                .show(ctx, |ui| spell_book(ui, stage, cast));
        }
    })
}

pub fn output_to_shapes(output: FullOutput) -> Vec<(Rect, TextureId)> {
    output
        .shapes
        .into_iter()
        .filter_map(|s| {
            if let ClippedShape {
                shape: Shape::Rect(rect_shape),
                ..
            } = s
            {
                Some(rect_shape)
            } else {
                None
            }
        })
        .filter_map(|rs| {
            if let RectShape {
                fill_texture_id: TextureId::User(_),
                ..
            } = rs
            {
                Some((rs.rect, rs.fill_texture_id))
            } else {
                None
            }
        })
        .collect()
}

fn menu(ui: &mut Ui, state: &mut Stage) {
    let buttons = [
        (Buttons::Settings, 3),
        (Buttons::Surrender, 54),
        (Buttons::Retreat, 105),
        (Buttons::AutoBattle, 156),
        (Buttons::BookOfMagic, 645),
        (Buttons::Wait, 696),
        (Buttons::Defend, 747),
    ];

    for (b, x) in buttons.into_iter() {
        let button = Button(b, crate::graphics::statics::ButtonState::Base);
        let texture_id: TextureId = button.into();
        let texture = egui::load::SizedTexture::new(texture_id, (48., 36.));
        let image = egui::widgets::Image::from_texture(texture);

        let button = egui::Button::image(image).frame(false);
        let rect = egui::Rect::from_two_pos(
            (x as f32, 557.).into(),
            ((x + 48) as f32, 557. + 44.).into(),
        );
        if ui.put(rect, button).clicked() {
            let name: &'static str = b.into();
            dbg!(name);

            if matches!(b, Buttons::BookOfMagic) {
                *state = Stage::SpellBook;
            }
        }
    }
}

fn spell_book(ui: &mut Ui, state: &mut Stage, command: &mut Option<Cast>) {
    let x_start = 210.;
    let y_start = 110.;

    let x_change = 80.;
    let y_change = 100.;

    for x in 0..2 {
        for y in 0..3 {
            let x_pos = x_start + x_change * x as f32;
            let y_pos = y_start + y_change * y as f32;

            let rect =
                egui::Rect::from_two_pos((x_pos, y_pos).into(), (x_pos + 67., y_pos + 48.).into());

            let texture_id = textures::convert_spell(Spell::Armageddon);
            let texture = egui::load::SizedTexture::new(texture_id, (67., 48.));
            let image = egui::widgets::Image::from_texture(texture);
            let button = egui::Button::image(image).frame(false);

            if ui.put(rect, button).clicked() {
                dbg!("ARMAGEDDON!!!");
                *state = Stage::Main;
                *command = Some(Cast {
                    spell: Spell::Armageddon,
                    target: None,
                });
            }
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
