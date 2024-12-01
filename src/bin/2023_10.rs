use std::fmt::Display;

const INPUT: &str = include_str!("../../resources/2023_10.txt");
const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, ""),
            Direction::East => write!(f, ""),
            Direction::South => write!(f, ""),
            Direction::West => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::NorthSouth),
            '-' => Ok(Self::EastWest),
            'L' => Ok(Self::NorthEast),
            'J' => Ok(Self::NorthWest),
            '7' => Ok(Self::SouthWest),
            'F' => Ok(Self::SouthEast),
            '.' => Ok(Self::Ground),
            'S' => Ok(Self::Start),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::NorthSouth => write!(f, "|"),
            Tile::EastWest => write!(f, "-"),
            Tile::NorthEast => write!(f, "L"),
            Tile::NorthWest => write!(f, "J"),
            Tile::SouthWest => write!(f, "7"),
            Tile::SouthEast => write!(f, "F"),
            Tile::Ground => write!(f, "."),
            Tile::Start => write!(f, "S"),
        }
    }
}

impl Tile {
    fn connects(&self, direction: &Direction) -> bool {
        match (self, direction) {
            (Tile::NorthSouth, Direction::North) => true,
            (Tile::NorthSouth, Direction::South) => true,
            (Tile::EastWest, Direction::West) => true,
            (Tile::EastWest, Direction::East) => true,
            (Tile::NorthWest, Direction::North) => true,
            (Tile::NorthWest, Direction::West) => true,
            (Tile::NorthEast, Direction::North) => true,
            (Tile::NorthEast, Direction::East) => true,
            (Tile::SouthWest, Direction::West) => true,
            (Tile::SouthWest, Direction::South) => true,
            (Tile::SouthEast, Direction::South) => true,
            (Tile::SouthEast, Direction::East) => true,
            (Tile::Start, Direction::North) => true,
            (Tile::Start, Direction::West) => true,
            (Tile::Start, Direction::South) => true,
            (Tile::Start, Direction::East) => true,
            _ => false,
        }
    }
}

struct Field {
    columns: usize,
    rows: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Field {
    fn move_direction(
        &self,
        direction: &Direction,
        position: (usize, usize),
    ) -> Option<(usize, usize)> {
        let (x, y) = position;
        match direction {
            Direction::North => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Direction::East => {
                if x + 1 < self.columns {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            Direction::South => {
                if y + 1 < self.rows {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            Direction::West => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
        }
    }

    fn get_tile(&self, position: (usize, usize)) -> &Tile {
        &self.tiles[position.0][position.1]
    }

    fn follow_trail(
        &self,
        previous: (usize, usize),
        position: (usize, usize),
        direction: &Direction,
        distance: u32,
    ) -> Option<u32> {
        let (x, y) = position;

        if self.get_tile(position) == &Tile::Start && distance > 0 {
            println!("{},{} reached Start with distance {}", x, y, distance);
            return Some(distance);
        }

        if !self.get_tile(previous).connects(&direction)
            || !self.get_tile(position).connects(&direction.opposite())
        {
            return None;
        }

        println!(
            "{} {} {} {}",
            self.get_tile(previous),
            direction,
            self.get_tile(position),
            distance
        );

        let valid_directions = DIRECTIONS
            .iter()
            .filter(|d| *d != &direction.opposite())
            .map(|d| (d, self.move_direction(d, position)))
            .filter_map(|(d, p)| p.map(|pos| (d, pos)));
        return valid_directions
            .filter_map(|(new_direction, new_position)| {
                self.follow_trail(position, new_position, &new_direction, distance + 1)
            })
            .max();
    }
}

fn main() {
    let columns = INPUT.find('\n').unwrap();
    let rows = INPUT.lines().count();

    let mut field = Field {
        columns,
        rows,
        tiles: vec![vec![Tile::Ground; rows]; columns],
    };
    let field_str: String = INPUT.chars().filter(|c| *c != '\n').collect();

    let start_col = field_str.find('S').unwrap() % columns;
    let start_row = field_str.find('S').unwrap() / rows;

    println!("columns: {}, rows: {}", columns, rows);
    println!("start: ({}, {})", start_col, start_row);

    for (i, c) in field_str.chars().enumerate() {
        if let Ok(tile) = Tile::try_from(c) {
            field.tiles[i % columns][i / columns] = tile;
        }
    }

    let start_position = (start_col, start_row);
    let valid_directions = DIRECTIONS
        .iter()
        .map(|d| (d, field.move_direction(d, start_position)))
        .filter_map(|(d, p)| p.map(|pos| (d, pos)));
    let distance = valid_directions
        .filter_map(|(new_direction, new_position)| {
            field.follow_trail(start_position, new_position, &new_direction, 0)
        })
        .max()
        .unwrap();
    println!("round trip distance: {}", distance);
    println!("furthest distance: {}", distance / 2 + 1);
}
