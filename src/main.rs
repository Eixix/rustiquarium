use std::cmp::max;
use std::thread::sleep;
use std::time;

use rand::random;

struct Fish {
    sprite: char,
    position: Point,
    movement: MovVec,
    move_cnt: u16,
    color: (u8, u8, u8),
}

struct Point {
    x: usize,
    y: usize,
}

struct MovVec {
    x_speed: i8,
    y_speed: i8,
}

fn main() {
    let terminal_width: usize;
    let terminal_height: usize;

    if let Some((w, h)) = term_size::dimensions() {
        terminal_width = w;
        terminal_height = h;
    } else {
        terminal_width = 1000;
        terminal_height = 1000;
    }

    let mut fishes: Vec<Fish> = spawn_fish(terminal_width, terminal_height);
    let mut clock_counter: u8 = 0;

    loop {
        // Clear terminal
        print!("{}[2J", 27 as char);
        sleep(time::Duration::from_millis(100));

        clock_counter = (clock_counter + 1) % 5;

        let mut aquarium: Vec<Vec<String>> = calculate_aquarium(terminal_width, terminal_height);

        for fish in &mut fishes {
            move_fish(fish, terminal_width, terminal_height, &clock_counter);
            // \x1b[38;2;R;G;Bm
            // \x1b[0m
            aquarium[fish.position.y][fish.position.x] = format!(
                "\x1b[38;2;{};{};{}m{}\x1b[0m",
                fish.color.0, fish.color.1, fish.color.2, fish.sprite
            );
        }

        for line in aquarium {
            println!("{}", String::from_iter(line))
        }
    }
}

fn calculate_aquarium(w: usize, h: usize) -> Vec<Vec<String>> {
    let bottom_top: Vec<String> = vec![String::from("="); w];

    let mut sides: Vec<String> = vec![String::from("|"); w];
    sides.splice(1..w - 1, vec![String::from(" "); w - 2]);

    let mut aquarium: Vec<Vec<String>> = vec![bottom_top; h - 1];
    aquarium.splice(1..h - 2, vec![sides; h - 3]);
    return aquarium;
}

fn move_fish(fish: &mut Fish, terminal_width: usize, terminal_height: usize, clock_counter: &u8) {
    fish.move_cnt += 1;
    fish.movement = calculate_movement(fish, terminal_width, terminal_height);

    if fish.position.x <= 1 || fish.position.x >= terminal_width - 2 {
        fish.movement.x_speed *= -1;
        fish.move_cnt = 0;
    }

    if fish.position.y <= 1 || fish.position.y >= terminal_height - 3 {
        fish.movement.y_speed *= -1;
        fish.move_cnt = 0;
    }

    if fish.movement.x_speed < 0 {
        fish.sprite = '<';
        fish.position.x -= 1
    } else if fish.movement.x_speed > 0 {
        fish.sprite = '>';
        fish.position.x += 1
    }
    if fish.movement.y_speed < 0 && *clock_counter == 0 && fish.movement.x_speed != 0 {
        fish.sprite = '<';
        fish.position.y -= 1
    } else if fish.movement.y_speed > 0 && *clock_counter == 0 && fish.movement.x_speed != 0 {
        fish.sprite = '>';
        fish.position.y += 1
    }
}

fn calculate_movement(fish: &mut Fish, terminal_width: usize, terminal_height: usize) -> MovVec {
    let direction_change_chance: f64 =
        0.5_f64 * (1_f64 + (0.1_f64 * (fish.move_cnt as f64) - 5_f64).tanh());
    let mut x_speed: i8 = fish.movement.x_speed;
    let mut y_speed: i8 = fish.movement.y_speed;

    if random::<f64>() < direction_change_chance
        && fish.position.x > 0
        && fish.position.x < terminal_width - 1
        && fish.position.y > 0
        && fish.position.y < terminal_height - 3
    {
        x_speed = calculate_speed(x_speed);
        y_speed = calculate_speed(y_speed);
        fish.move_cnt = 0;
    }

    return MovVec { x_speed, y_speed };
}

fn calculate_speed(initial_speed: i8) -> i8 {
    let random_chance: f64 = random::<f64>();
    return if random_chance < 0.33 {
        max(initial_speed, 1) * -1
    } else if random_chance < 0.66 {
        0
    } else {
        max(initial_speed, 1) * 1
    };
}

fn spawn_fish(terminal_width: usize, terminal_height: usize) -> Vec<Fish> {
    let mut to_return: Vec<Fish> = vec![];

    for i in 0..100 {
        let fish = Fish {
            sprite: '>',
            position: Point {
                x: std::cmp::min(
                    max(
                        (random::<f64>() * (terminal_width as f64) - 1_f64) as usize,
                        1,
                    ),
                    terminal_width - 1,
                ),
                y: std::cmp::min(
                    max(
                        (random::<f64>() * (terminal_height as f64) - 1_f64) as usize,
                        1,
                    ),
                    terminal_height - 3,
                ),
            },
            movement: MovVec {
                x_speed: calculate_speed(0),
                y_speed: calculate_speed(0),
            },
            move_cnt: 0,
            color: (random::<u8>(), random::<u8>(), random::<u8>()),
        };

        to_return.push(fish);
    }
    return to_return;
}
