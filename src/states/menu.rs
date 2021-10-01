use bevy::{app::AppExit, prelude::*};

use crate::{helper::cleanup_system, AppState, ButtonMaterials};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Menu).with_system(menu_interaction_system),
            )
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(exit_system))
            .add_system_set(
                SystemSet::on_exit(AppState::Menu).with_system(cleanup_system::<MenuCleanup>),
            );
    }
}

struct MenuCleanup;

enum MenuButton {
    Forest,
    Game,
    Exit,
}
impl MenuButton {
    fn name(&self) -> String {
        match self {
            Self::Forest => "Forest".to_string(),
            Self::Game => "Game".to_string(),
            Self::Exit => "Exit".to_string(),
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    // ui camera
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(MenuCleanup);

    // Create buttons
    let style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };

    let _button_forest =
        create_button(&mut commands, &button_materials, &style, MenuButton::Forest);
    let _button_game = create_button(&mut commands, &button_materials, &style, MenuButton::Game);
    let _button_exit = create_button(&mut commands, &button_materials, &style, MenuButton::Exit);
}

fn create_button(
    commands: &mut Commands,
    button_materials: &ButtonMaterials,
    style: &TextStyle,
    button: MenuButton,
) -> Entity {
    let button_entity = commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(button.name(), style.clone(), Default::default()),
                ..Default::default()
            });
        })
        .insert(Name::new(format!("{} Button", button.name())))
        .insert(button)
        .insert(MenuCleanup)
        .id();
    button_entity
}

fn menu_interaction_system(
    mut state: ResMut<State<AppState>>,
    button_materials: Res<ButtonMaterials>,
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
                    MenuButton::Forest => state.set(AppState::Forest).unwrap(),
                    MenuButton::Game => state.set(AppState::Game).unwrap(),
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
