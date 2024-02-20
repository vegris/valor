pub mod image_button;
pub mod textures;

use egui::{
    epaint::{ClippedShape, RectShape},
    Context, FullOutput, RawInput, Rect, Shape, TextureId, Ui,
};
use gamedata::spells::Spell;

use crate::{command::Cast, graphics::statics::Buttons, input::FrameInput, State};

use self::textures::Button;

pub fn create_context() -> Context {
    Context::default()
}

pub fn create_frame(
    ctx: &Context,
    input: &FrameInput,
    state: &mut State,
    cast: &mut Option<Cast>,
) -> FullOutput {
    let raw_input = raw_input_from_frame_input(input);

    ctx.run(raw_input, |ctx| {
        egui::Area::new("menu")
            .fixed_pos((1., 555.))
            .show(ctx, |ui| menu(ui, state));

        if matches!(state, State::SpellBook) {
            egui::Area::new("spellbook")
                .fixed_pos((400. - 310., 0.))
                .show(ctx, |ui| spell_book(ui, state, cast));
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

fn menu(ui: &mut Ui, state: &mut State) {
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
        let button = Button(b, crate::graphics::statics::ButtonState::Base);
        let texture_id: TextureId = button.into();
        let texture = egui::load::SizedTexture::new(texture_id, (48., 36.));
        let image = egui::widgets::Image::from_texture(texture);

        let button = egui::Button::image(image).frame(false);
        let rect = egui::Rect::from_two_pos(
            (x as f32, 555.).into(),
            ((x + 48) as f32, 555. + 44.).into(),
        );
        if ui.put(rect, button).clicked() {
            let name: &'static str = b.into();
            dbg!(name);

            if matches!(b, Buttons::BookOfMagic) {
                *state = State::SpellBook;
            }
        }
    }
}

fn spell_book(ui: &mut Ui, state: &mut State, command: &mut Option<Cast>) {
    let texture_id = textures::convert_spell(Spell::Armaggedon);
    let texture = egui::load::SizedTexture::new(texture_id, (67., 48.));
    let image = egui::widgets::Image::from_texture(texture);
    let button = egui::Button::image(image).frame(false);

    let x_pos = 210.;
    let y_pos = 110.;

    let rect = egui::Rect::from_two_pos((x_pos, y_pos).into(), (x_pos + 67., y_pos + 48.).into());
    if ui.put(rect, button).clicked() {
        dbg!("ARMAGEDDON!!!");
        *state = State::Main;
        *command = Some(Cast {
            spell: Spell::Armaggedon,
            target: None,
        });
    }

    let texture_id = textures::convert_spell(Spell::Armaggedon);
    let texture = egui::load::SizedTexture::new(texture_id, (67., 48.));
    let image = egui::widgets::Image::from_texture(texture);
    let button = egui::Button::image(image).frame(false);

    let x_pos = 290.;
    let y_pos = 110.;

    let rect = egui::Rect::from_two_pos((x_pos, y_pos).into(), (x_pos + 67., y_pos + 48.).into());
    if ui.put(rect, button).clicked() {
        dbg!("ARMAGEDDON!!!");
        *state = State::Main;
    }

    let texture_id = textures::convert_spell(Spell::Armaggedon);
    let texture = egui::load::SizedTexture::new(texture_id, (67., 48.));
    let image = egui::widgets::Image::from_texture(texture);
    let button = egui::Button::image(image).frame(false);

    let x_pos = 210.;
    let y_pos = 210.;

    let rect = egui::Rect::from_two_pos((x_pos, y_pos).into(), (x_pos + 67., y_pos + 48.).into());
    if ui.put(rect, button).clicked() {
        dbg!("ARMAGEDDON!!!");
        *state = State::Main;
    }

    let texture_id = textures::convert_spell(Spell::Armaggedon);
    let texture = egui::load::SizedTexture::new(texture_id, (67., 48.));
    let image = egui::widgets::Image::from_texture(texture);
    let button = egui::Button::image(image).frame(false);

    let x_pos = 290.;
    let y_pos = 210.;

    let rect = egui::Rect::from_two_pos((x_pos, y_pos).into(), (x_pos + 67., y_pos + 48.).into());
    if ui.put(rect, button).clicked() {
        dbg!("ARMAGEDDON!!!");
        *state = State::Main;
    }

    let texture_id = textures::convert_spell(Spell::Armaggedon);
    let texture = egui::load::SizedTexture::new(texture_id, (67., 48.));
    let image = egui::widgets::Image::from_texture(texture);
    let button = egui::Button::image(image).frame(false);

    let x_pos = 210.;
    let y_pos = 310.;

    let rect = egui::Rect::from_two_pos((x_pos, y_pos).into(), (x_pos + 67., y_pos + 48.).into());
    if ui.put(rect, button).clicked() {
        dbg!("ARMAGEDDON!!!");
        *state = State::Main;
    }

    let texture_id = textures::convert_spell(Spell::Armaggedon);
    let texture = egui::load::SizedTexture::new(texture_id, (67., 48.));
    let image = egui::widgets::Image::from_texture(texture);
    let button = egui::Button::image(image).frame(false);

    let x_pos = 290.;
    let y_pos = 310.;

    let rect = egui::Rect::from_two_pos((x_pos, y_pos).into(), (x_pos + 67., y_pos + 48.).into());
    if ui.put(rect, button).clicked() {
        dbg!("ARMAGEDDON!!!");
        *state = State::Main;
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
