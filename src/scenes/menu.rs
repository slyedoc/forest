use crate::{helper::cleanup_system, style::ButtonAssets, AppState};
use bevy::{app::AppExit, prelude::*};

/// Main Menu
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

#[derive(Component)]
struct MenuCleanup;

#[derive(Component)]
enum MenuButton {
    Forest,
    TreeTest,
    TurtleTest,
    Exit,
}

impl MenuButton {
    fn name(&self) -> &str {
        match self {
            MenuButton::Forest => "Forest",
            MenuButton::TreeTest => "Tree Test",
            MenuButton::TurtleTest => "Turtle Test",
            MenuButton::Exit => "Exit",
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            create_button(parent, &button_materials, &style, MenuButton::Forest);
            create_button(parent, &button_materials, &style, MenuButton::TreeTest);
            create_button(parent, &button_materials, &style, MenuButton::TurtleTest);
            create_button(parent, &button_materials, &style, MenuButton::Exit);
        })
        .insert(Name::new("Menu"))
        .insert(MenuCleanup);
}

fn create_button(
    parent: &mut ChildBuilder,
    button_materials: &ButtonAssets,
    style: &TextStyle,
    button: MenuButton,
) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
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
        .insert(button);
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
                    MenuButton::Forest => state.set(AppState::Forest).unwrap(),
                    MenuButton::TreeTest => state.set(AppState::TreeTest).unwrap(),
                    MenuButton::TurtleTest => state.set(AppState::TurtleTest).unwrap(),
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
