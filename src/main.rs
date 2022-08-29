use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .add_system(jump_player)
        .run();
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
    Jump,
}

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .insert(GamepadButtonType::South, Action::Jump)
                .build(),
        });
}

fn move_player(query: Query<&ActionState<Action>, With<Player>>) {
    let action_state = query.single();
    if action_state.pressed(Action::Move) {
        let axis_pair = action_state.clamped_axis_pair(Action::Move).unwrap();
        println!("Move:");
        println!("Distance: {}", axis_pair.length());
        println!("X: {}", axis_pair.x());
        println!("Y: {}", axis_pair.y());
    }
}

fn jump_player(query: Query<&ActionState<Action>, With<Player>>) {
    let action_state = query.single();
    if action_state.just_pressed(Action::Jump) {
        println!("Jump");
    }
}
