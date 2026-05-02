use bevy::prelude::*;
use bracket_bevy::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50().with_random_number_generator(true))
        .add_startup_system(setup)
        .add_system(tick)
        .run();
}

fn setup(mut commands: Commands, rng: Res<RandomNumbers>) {
    let (rooms, map) = new_map_rooms_and_corridors(&rng);
    let (player_x, player_y) = rooms[0].center();
    commands.insert_resource(map);
    commands
        .spawn_empty()
        .insert(Position {
            x: player_x,
            y: player_y,
        })
        .insert(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .insert(Player {});
}

fn tick(
    ctx: Res<BracketContext>,
    map: Res<Map>,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Position, &Renderable), With<Player>>,
) {
    ctx.cls();

    let delta = player_input(&keyboard);
    let (mut pos, render) = player_query.single_mut();
    if delta != (0, 0) {
        let destination_idx = xy_idx(pos.x + delta.0, pos.y + delta.1);
        if map.0[destination_idx] != TileType::Wall {
            pos.x = (pos.x + delta.0).clamp(0, 79);
            pos.y = (pos.y + delta.1).clamp(0, 49);
        }
    }

    draw_map(&map.0, &ctx);
    ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
}
