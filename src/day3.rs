#[derive(PartialEq, Debug, Eq, PartialOrd, Ord)]
pub struct Vec2D {
    pub x: usize,
    pub y: usize,
}

pub struct Map {
    pub locations: Vec<Vec<char>>,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        Map {
            locations: s
                .lines()
                .map(|line| line.trim().chars().collect())
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TileType {
    Empty,
    Symbol,
    Digit,
    OOB,
}

impl Map {
    pub fn get_tile_type(&self, x: usize, y: usize) -> TileType {
        // check if it's any symbol but not a dot
        match self.get(x, y) {
            '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '+' | '=' | '/' => {
                TileType::Symbol
            }
            '0'..='9' => TileType::Digit,
            '.' => TileType::Empty,
            _ => panic!("Invalid tile type at {}, {} : {}", x, y, self.get(x, y)),
        }
    }

    pub fn get_start_of_number(&self, x: usize, y: usize) -> Vec2D {
        let mut start = Vec2D { x, y };

        // go left until we hit the edge (x=0) or a symbol
        while start.x > 0 && self.get_tile_type(start.x - 1, start.y) == TileType::Digit {
            start.x -= 1;
        }

        start
    }

    pub fn read_number(&self, x: usize, y: usize) -> u32 {
        // starting at x,y. Go right and read the digits, until we hit the edge (x=width) or a symbol
        let mut number = String::new();
        let mut x = x;

        while x < self.width() && self.get_tile_type(x, y) == TileType::Digit {
            number.push(self.get(x, y));
            x += 1;
        }

        number.parse::<u32>().unwrap()
    }

    pub fn list_symbols(&self) -> Vec<Vec2D> {
        let mut symbols = Vec::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.get_tile_type(x, y) == TileType::Symbol {
                    symbols.push(Vec2D { x, y });
                }
            }
        }

        symbols
    }

    pub fn width(&self) -> usize {
        self.locations[0].len()
    }

    pub fn height(&self) -> usize {
        self.locations.len()
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.locations[y][x]
    }

    pub fn get_valid_surrounding_vecs(&self, x: usize, y: usize) -> Vec<Vec2D> {
        let mut vecs = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                if new_x >= 0
                    && new_x < self.width() as i32
                    && new_y >= 0
                    && new_y < self.height() as i32
                {
                    vecs.push(Vec2D {
                        x: new_x as usize,
                        y: new_y as usize,
                    });
                }
            }
        }

        vecs
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    Map::from(input)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Map) -> u32 {
    let mut numbers: Vec<Vec2D> = input
        .list_symbols()
        .iter()
        .flat_map(|symbol| {
            input
                .get_valid_surrounding_vecs(symbol.x, symbol.y)
                .into_iter()
                .filter(|vec| input.get_tile_type(vec.x, vec.y) == TileType::Digit)
                .map(|vec| input.get_start_of_number(vec.x, vec.y))
        })
        .collect();

    numbers.sort();
    numbers.dedup();

    numbers
        .iter()
        .map(|number| input.read_number(number.x, number.y))
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Map) -> u32 {
    let binding = input.list_symbols();

    let result = binding
        .iter()
        .filter(|s| input.get(s.x, s.y) == '*')
        .filter_map(|g| {
            let mut surrounding_numbers = input
                .get_valid_surrounding_vecs(g.x, g.y)
                .into_iter()
                .filter(|vec| input.get_tile_type(vec.x, vec.y) == TileType::Digit)
                .map(|vec| input.get_start_of_number(vec.x, vec.y))
                .collect::<Vec<Vec2D>>();

            surrounding_numbers.sort();
            surrounding_numbers.dedup();

            if surrounding_numbers.len() == 2 {
                let number1 = input.read_number(surrounding_numbers[0].x, surrounding_numbers[0].y);
                let number2 = input.read_number(surrounding_numbers[1].x, surrounding_numbers[1].y);
                Some(number1 * number2)
            } else {
                None
            }
        })
        .fold(0, |acc, product| acc + product);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "467..114..
                      ...*......
                      ..35..633.
                      ......#...
                      617*......
                      .....+.58.
                      ..592.....
                      ......755.
                      ...$.*....
                      .664.598..";

    #[test]
    fn test_part1() {
        let input = Map::from(EXAMPLE1);

        assert_eq!(solve_part1(&input), 4361);
    }

    #[test]
    fn test_part2() {
        let input = Map::from(EXAMPLE1);

        assert_eq!(solve_part2(&input), 467835);
    }

    #[test]
    fn test_get_start_of_number() {
        let input = Map::from(EXAMPLE1);

        assert_eq!(input.get_start_of_number(1, 0), Vec2D { x: 0, y: 0 });
        assert_eq!(input.get_start_of_number(6, 9), Vec2D { x: 5, y: 9 });
    }

    #[test]
    fn test_read_number() {
        let input = Map::from(EXAMPLE1);

        assert_eq!(input.read_number(0, 0), 467);
        assert_eq!(input.read_number(5, 9), 598);
    }
}
