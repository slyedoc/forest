use bevy::{app::AppExit, prelude::*};
use strum_macros::{AsRefStr, EnumIter};
use strum::IntoEnumIterator;

use crate::prelude::*;

/// Main Menu
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Menu)
                .with_system(menu_interaction_system)
                .with_system(exit_system),
            )
            .add_system_set(SystemSet::on_update(AppState::Menu))
            .add_system_set(
                SystemSet::on_exit(AppState::Menu).with_system(clear_system),
            );
    }
}

#[derive(Component, EnumIter, AsRefStr)]
enum MenuButton {
    #[strum(serialize = "Spider Attack")]
    SpiderAttack,

    #[strum(serialize = "Solar System")]
    SolarSystem,

    #[strum(serialize = "Astroid")]
    Astroid,

    #[strum(serialize = "LSystem Test")]
    LSystemTest,
    #[strum(serialize = "Space Asset Preview")]
    SpaceAssetPreview,
    #[strum(serialize = "City Asset Preview")]
    CityAssetPreview,
    Exit,
}

fn setup(
    mut commands: Commands,
    button_assets: Res<ButtonAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Create buttons
    let style = TextStyle {
        font: button_assets.fira_sans.clone(),
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            for b in MenuButton::iter() {
                parent.spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(500.0), Val::Px(65.0)),
                        margin: Rect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_assets.normal.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(   b.as_ref(), style.clone(), Default::default()),
                        ..Default::default()
                    });
                })
                .insert(Name::new(format!("{} Button", b.as_ref())))
                .insert(b);
            }
        })
        .insert(Name::new("Menu"));
}

fn menu_interaction_system(
    mut state: ResMut<State<AppState>>,
    button_materials: Res<ButtonAssets>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &MenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, mut material, button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                match *button {
                    MenuButton::SolarSystem => state.set(AppState::SolarSystem).unwrap(),
                    MenuButton::Astroid => state.set(AppState::Astroid).unwrap(),
                    MenuButton::LSystemTest => state.set(AppState::LSystemTest).unwrap(),
                    MenuButton::SpaceAssetPreview => state.set(AppState::SpaceAssetPreview).unwrap(),
                    MenuButton::CityAssetPreview => state.set(AppState::CityAssetPreview).unwrap(),
                    MenuButton::SpiderAttack => state.set(AppState::SpiderAttack).unwrap(),
                    MenuButton::Exit => exit.send(AppExit),
                }
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn exit_system(mut exit: EventWriter<AppExit>, keys: Res<Input<KeyCode>>) {
    if keys.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
