use infra::common::direction::Direction;
use infra::common::matrix::Matrix;

use rand::Rng;

/// A cell in a maze.
pub trait Cell {
    fn set_wall(&mut self, Direction);
}

/// Generate a maze.
pub fn generate<R, T>(rng: &mut R, matrix: &mut Matrix<T>)
    where R: Rng, T: Cell {
    let (width, height) = (matrix.width(), matrix.height());
    if width == 0 || height == 0 {
        return;
    }
    recursive(rng, matrix, false, 0, width as isize, 0, height as isize);
    for x in 0 .. width {
        matrix[(x,          0)].set_wall(Direction::North);
        matrix[(x, height - 1)].set_wall(Direction::South);
    }
    for y in 0 .. height {
        matrix[(        0, y)].set_wall(Direction::West);
        matrix[(width - 1, y)].set_wall(Direction::East);
    }
}

fn recursive<R, T>(rng: &mut R, matrix: &mut Matrix<T>, vertical: bool,
                   x1: isize, x2: isize, y1: isize, y2: isize)
    where R: Rng, T: Cell {
    if x2 - x1 < 2 || y2 - y1 < 2 {
        return;
    }
    if vertical {
        let wall_x = recursive_helper(rng, matrix, vertical, x1, x2, y1, y2);
        recursive(rng, matrix, !vertical, x1, wall_x, y1, y2);
        recursive(rng, matrix, !vertical, wall_x, x2, y1, y2);
    } else {
        let wall_y = recursive_helper(rng, matrix, vertical, y1, y2, x1, x2);
        recursive(rng, matrix, !vertical, x1, x2, y1, wall_y);
        recursive(rng, matrix, !vertical, x1, x2, wall_y, y2);
    }
}

fn recursive_helper<R, T>(rng: &mut R, matrix: &mut Matrix<T>, vertical: bool,
                          c1: isize, c2: isize, d1: isize, d2: isize) -> isize
    where R: Rng, T: Cell {
    let wall_c = if c2 - c1 < 32 {
        rng.gen_range(c1 + 1, c2)
    } else {
        c1 + (c2 - c1) / 2 + rng.gen_range(-4, 4)
    };
    let pass_d = rng.gen_range(d1 + 1, d2);
    for wall_d in d1 .. d2 {
        if wall_d != pass_d && !rng.gen_weighted_bool(6) {
            if vertical {
                for cell in matrix.get_mut(wall_c as usize + 0, wall_d as usize) {
                    cell.set_wall(Direction::East);
                }
                for cell in matrix.get_mut(wall_c as usize + 1, wall_d as usize) {
                    cell.set_wall(Direction::West);
                }
            } else {
                for cell in matrix.get_mut(wall_d as usize + 0, wall_c as usize) {
                    cell.set_wall(Direction::South);
                }
                for cell in matrix.get_mut(wall_d as usize + 1, wall_c as usize) {
                    cell.set_wall(Direction::North);
                }
            }
        }
    }
    wall_c
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand;

    #[derive(Clone, Copy, Debug)]
    struct SimpleCell {
        walls: u8,
    }

    impl SimpleCell {
        pub fn new() -> Self {
            SimpleCell{walls: 0b0000}
        }

        pub fn wall(&self, wall: Direction) -> bool {
            self.walls & Self::wall_mask(wall) != 0
        }

        fn wall_mask(wall: Direction) -> u8 {
            match wall {
                Direction::North => 0b0001,
                Direction::South => 0b0010,
                Direction::West  => 0b0100,
                Direction::East  => 0b1000,
            }
        }
    }

    impl Cell for SimpleCell {
        fn set_wall(&mut self, wall: Direction) {
            self.walls |= Self::wall_mask(wall);
        }
    }

    #[test]
    fn test_generate() {
        let mut rng = rand::thread_rng();
        let mut maze = Matrix::new(SimpleCell::new(), 256, 128);
        generate(&mut rng, &mut maze);

        let cell_size = 4;
        println!("<?xml version='1.0' ?>");
        println!("<svg xmlns='http://www.w3.org/2000/svg' width='{}' height='{}' shape-rendering='crispEdges'>",
                 maze.width() * cell_size, maze.height() * cell_size);
        for y in 0 .. maze.height() {
            for x in 0 .. maze.width() {
                let cell = maze[(x, y)];
                if cell.wall(Direction::West) {
                    println!("<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='black' />",
                                x * cell_size, x * cell_size,
                                y * cell_size, (y + 1) * cell_size);
                }
                if cell.wall(Direction::North) {
                    println!("<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='black' />",
                                x * cell_size, (x + 1) * cell_size,
                                y * cell_size, y * cell_size);
                }
            }
        }
        println!("</svg>");
    }
}
