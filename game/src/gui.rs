use egui::epaint::{ClippedShape, RectShape};
use egui::{Context, FullOutput, Shape, TextureId, Ui};
use gamedata::gui::{Button, ButtonState, Texture};
use gamedata::spells::Spell;
use logic::command::Cast;
use sdl2::rect::Rect;

use crate::input::FrameInput;
use crate::{input, Stage};

pub fn create_frame(
    ctx: &Context,
    input: &FrameInput,
    stage: &mut Stage,
    cast: &mut Option<Cast>,
) -> Vec<(Rect, Texture)> {
    let raw_input = input::to_raw_input(input);

    let full_output = ctx.run(raw_input, |ctx| {
        egui::Area::new("menu")
            .fixed_pos((0., 556.))
            .show(ctx, |ui| menu(ui, stage));

        if matches!(stage, Stage::SpellBook) {
            egui::Area::new("spellbook")
                .fixed_pos((400. - 310., 0.))
                .show(ctx, |ui| spell_book(ui, stage, cast));
        }
    });

    to_shapes(full_output)
}

fn to_shapes(output: FullOutput) -> Vec<(Rect, Texture)> {
    output
        .shapes
        .into_iter()
        .filter_map(|s| match s {
            ClippedShape {
                shape:
                    Shape::Rect(RectShape {
                        fill_texture_id: TextureId::User(texture_id),
                        rect,
                        ..
                    }),
                ..
            } => {
                let rect = Rect::new(
                    rect.min.x as i32,
                    rect.min.y as i32,
                    rect.width() as u32,
                    rect.height() as u32,
                );
                let texture = texture_id.try_into().unwrap();
                Some((rect, texture))
            }
            _ => None,
        })
        .collect()
}

fn menu(ui: &mut Ui, state: &mut Stage) {
    let buttons = [
        (Button::Settings, 3),
        (Button::Surrender, 54),
        (Button::Retreat, 105),
        (Button::AutoBattle, 156),
        (Button::BookOfMagic, 645),
        (Button::Wait, 696),
        (Button::Defend, 747),
    ];

    for (b, x) in buttons.into_iter() {
        let button = Texture::Button(b, ButtonState::Base);
        let texture_id = TextureId::User(button.into());
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

            if matches!(b, Button::BookOfMagic) {
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

            let texture_id = TextureId::User(Texture::Spell(Spell::Armageddon).into());
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
