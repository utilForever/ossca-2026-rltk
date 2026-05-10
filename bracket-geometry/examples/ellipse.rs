use bracket_geometry::prelude::*;
use crossterm::queue;
use crossterm::style::Print;
use std::collections::HashSet;
use std::io::{stdout, Write};

const WIDTH: usize = 20;
const HEIGHT: usize = 10;

fn main() {
    let mut fake_console = vec!['.'; WIDTH * HEIGHT];
    let cloud_center = Point::new(10, 5);
    let player_position = Point::new(10, 5);
    let monster_position = Point::new(14, 6);
    let outside_monster_position = Point::new(18, 1);
    let poison_cloud: HashSet<Point> = ellipse2d(cloud_center, 6, 3).into_iter().collect();

    for point in &poison_cloud {
        draw_point(&mut fake_console, *point, '*');
    }

    draw_point(&mut fake_console, player_position, '@');
    draw_point(&mut fake_console, monster_position, 'M');
    draw_point(&mut fake_console, outside_monster_position, 'm');

    print_map(&fake_console);

    let player_in_cloud = poison_cloud.contains(&player_position);
    let monster_in_cloud = poison_cloud.contains(&monster_position);
    let outside_monster_in_cloud = poison_cloud.contains(&outside_monster_position);

    queue!(
        stdout(),
        Print(format!(
            "\n@ in cloud: {player_in_cloud}\nM in cloud: {monster_in_cloud}\nm in cloud: {outside_monster_in_cloud}\n"
        ))
    )
    .expect("Command fail");
    stdout().flush().expect("Flush Fail");
}

fn draw_point(fake_console: &mut [char], point: Point, glyph: char) {
    if point.x >= 0 && point.x < WIDTH as i32 && point.y >= 0 && point.y < HEIGHT as i32 {
        let idx = point.y as usize * WIDTH + point.x as usize;
        fake_console[idx] = glyph;
    }
}

fn print_map(fake_console: &[char]) {
    for y in 0..HEIGHT {
        let mut line = String::new();
        let idx = y * WIDTH;
        for x in 0..WIDTH {
            line.push(fake_console[idx + x]);
        }
        line.push('\n');
        queue!(stdout(), Print(&line)).expect("Command fail");
    }
}
