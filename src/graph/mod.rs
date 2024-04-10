mod generation;
mod graph_wrapper;

pub(crate) use generation::Generation;
pub(crate) use graph_wrapper::GraphWrapper;

#[derive(Clone, PartialEq, Eq)]
// #[rustfmt::skip]
// pub enum Point {
//     Start    { x: usize, y: usize },
//     End      { x: usize, y: usize },
//     Free     { x: usize, y: usize },
//     Obstacle { x: usize, y: usize, length: u32 },
//     Path     { x: usize, y: usize, initial_point: Box<Point> },
//     Seen     { x: usize, y: usize, initial_point: Box<Point> },
// }
pub enum Point {
    Start,
    End,
    Free,
    Path { initial_point: Box<Point> },
    Seen { initial_point: Box<Point> },
    Obstacle { length: u32 },
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointCoord {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone)]
pub struct Graph {
    mtx: Vec<Vec<Point>>,
}

impl Graph {
    const DIRS: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
}

impl Graph {
    pub fn reset(&mut self) {
        for row in &mut self.mtx {
            for point in row {
                match point {
                    Point::Path { initial_point } => *point = *initial_point.clone(),
                    Point::Seen { initial_point } => *point = *initial_point.clone(),
                    _ => {}
                }
            }
        }
    }

    pub fn point_at_mut<'a>(&'a mut self, point_coord: &PointCoord) -> &'a mut Point {
        &mut self.mtx[point_coord.y][point_coord.x]
    }

    pub fn point_at<'a>(&'a self, point_coord: &PointCoord) -> &'a Point {
        &self.mtx[point_coord.y][point_coord.x]
    }

    pub fn neighbors<'a, 'b>(
        &'a self,
        point_coord: &'b PointCoord,
    ) -> impl Iterator<Item = (&'a Point, PointCoord)> + 'b
    where
        'a: 'b,
    {
        Self::DIRS.iter().filter_map(move |(di, dj)| {
            let to_i = (point_coord.y as i32 + di) as usize;
            let to_j = (point_coord.x as i32 + dj) as usize;

            let point = self.mtx.get(to_i)?.get(to_j)?;

            Some((point, PointCoord { y: to_i, x: to_j }))
        })
    }

    pub fn n(&self) -> usize {
        self.mtx.len()
    }

    pub fn m(&self) -> usize {
        self.mtx[0].len()
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Point::Start => "S",
                Point::End => "E",
                Point::Free => ".",
                Point::Path { .. } => "*",
                Point::Obstacle { .. } => "X",
                Point::Seen { .. } => "O",
            },
        )
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.mtx {
            for point in row {
                write!(f, "{}", point)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Graph {
    pub const N: usize = 10;
    pub const M: usize = 10;

    pub fn start(&self) -> Option<PointCoord> {
        let mut start: Option<PointCoord> = None;

        'out: for (i_, row_) in self.mtx.iter().enumerate() {
            for (j_, point) in row_.iter().enumerate() {
                if point == &Point::Start {
                    start = Some(PointCoord { x: j_, y: i_ });
                    break 'out;
                }
            }
        }

        start
    }

    pub fn end(&self) -> Option<PointCoord> {
        let mut end: Option<PointCoord> = None;

        'out: for (i_, row_) in self.mtx.iter().enumerate() {
            for (j_, point) in row_.iter().enumerate() {
                if point == &Point::End {
                    end = Some(PointCoord { x: j_, y: i_ });
                    break 'out;
                }
            }
        }

        end
    }
}
