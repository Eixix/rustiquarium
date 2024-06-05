use std::cmp::PartialEq;
use std::thread::sleep;
use std::time;

struct Fish {
    sprite: char,
    x: usize,
    y: usize,
    direction: Direction
}

enum Direction {
    Left,
    Right
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right))
    }
}

fn main() {
    let mut terminal_width: usize;
    let mut terminal_height: usize;

    if let Some((w, h)) = term_size::dimensions() {
        terminal_width = w;
        terminal_height = h;
    } else {
        panic!("Unable to get term size");
    }

    let mut fish = Fish {
        sprite: '>',
        x: (terminal_width / 2),
        y: (terminal_height / 2),
        direction: Direction::Right,
    };

    loop {
        // Clear terminal
        print!("{}[2J", 27 as char);
        sleep(time::Duration::from_millis(100));

        let mut aquarium: Vec<Vec<char>> = calculate_aquarium(terminal_width, terminal_height);

        move_fish(&mut fish, terminal_width, terminal_height);

        aquarium[fish.y][fish.x] = fish.sprite;

        for line in aquarium {
            println!("{}", String::from_iter(line))
        }
    }
}

fn calculate_aquarium(w: usize, h: usize) -> Vec<Vec<char>> {
    let bottom_top: Vec<char> = vec!['='; w];

    let mut sides: Vec<char> = vec!['|'; w];
    sides.splice(1..w - 1, vec![' '; w - 2]);

    let mut aquarium: Vec<Vec<char>> = vec![bottom_top; h - 1];
    aquarium.splice(1..h - 2, vec![sides; h - 3]);
    return aquarium;
}


fn move_fish(fish: &mut Fish, terminal_width: usize, terminal_height: usize) {
    if fish.direction == Direction::Left {
        if (fish.x <= 1) {
            fish.direction = Direction::Right;
            fish.sprite = '>';
        } else {
            fish.x -= 1
        }
    } else {
        if (fish.x >= terminal_width - 2) {
            fish.direction = Direction::Left;
            fish.sprite = '<';
        } else {
            fish.x += 1
        }
    }
}