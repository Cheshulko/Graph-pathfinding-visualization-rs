use rand::Rng;

#[derive(Clone, PartialEq, Eq)]
pub enum Point {
    Start,
    End,
    Free,
    Path { initial_point: Box<Point> },
    Seen { initial_point: Box<Point> },
    Obstacle { length: u32 },
}

pub struct Graph {
    pub mtx: Vec<Vec<Point>>,
}

pub enum Generation {
    Predefined1,
    Random,
}

impl Graph {
    const DIRS: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn generate_graph(generation: Generation) -> Self {
        match generation {
            Generation::Predefined1 => Graph::generate_graph_predefined_1(),
            Generation::Random => Graph::generate_random(),
        }
    }

    fn generate_graph_predefined_1() -> Self {
        println!("[I] generate_graph_predefined_1");

        let mut mtx = vec![vec![Point::Free; Self::M]; Self::N];

        mtx[0][0] = Point::Start;
        mtx[0][Self::M - 1] = Point::End;

        for i in 0..Self::N - 1 {
            mtx[i][3] = Point::Obstacle { length: 3 };
        }
        mtx[2][3] = Point::Obstacle { length: 1 };

        mtx[1][0] = Point::Obstacle { length: 0 };
        mtx[1][1] = Point::Obstacle { length: 1 };
        mtx[3][2] = Point::Obstacle { length: 2 };
        mtx[3][1] = Point::Obstacle { length: 3 };

        mtx[3][Self::M - 1] = Point::Obstacle { length: 0 };
        mtx[3][Self::M - 2] = Point::Obstacle { length: 1 };
        mtx[3][Self::M - 3] = Point::Obstacle { length: 2 };
        mtx[3][Self::M - 4] = Point::Obstacle { length: 3 };

        mtx[7][4] = Point::Obstacle { length: 1 };
        mtx[7][5] = Point::Obstacle { length: 2 };
        mtx[7][6] = Point::Obstacle { length: 3 };

        Graph { mtx }
    }

    fn generate_random() -> Self {
        println!("[I] generate_random");

        let mut mtx = vec![vec![Point::Free; Self::M]; Self::N];

        let mut rng = rand::thread_rng();

        let obstacles_cnt = rng.gen_range(1..=(Self::M * Self::N) / 2);
        let mut obstacles_generated = 0;

        while obstacles_generated < obstacles_cnt {
            let (i, j) = (rng.gen_range(0..Self::N), rng.gen_range(0..Self::M));

            if mtx[i][j] == Point::Free {
                mtx[i][j] = Point::Obstacle { length: 1 };
                obstacles_generated += 1;
            }
        }

        let (start_i, start_j) = loop {
            let (start_i, start_j) = (rng.gen_range(0..Self::N), rng.gen_range(0..Self::M));

            if mtx[start_i][start_j] == Point::Free {
                mtx[start_i][start_j] = Point::Start;
                break (start_i, start_j);
            }
        };

        loop {
            let (end_i, end_j) = (rng.gen_range(0..Self::N), rng.gen_range(0..Self::M));

            if start_i.max(end_i) - start_i.min(end_i) + start_j.max(end_j) - start_j.min(end_j)
                < (Self::N + Self::M) / 2
            {
                continue;
            }

            if mtx[end_i][end_j] == Point::Free {
                mtx[end_i][end_j] = Point::End;
                break;
            }
        }

        Graph { mtx }
    }

    pub fn neighbors<'a>(
        &'a self,
        cur_i: usize,
        cur_j: usize,
    ) -> impl Iterator<Item = (&'a Point, usize, usize)> {
        Self::DIRS.iter().filter_map(move |(di, dj)| {
            let to_i = (cur_i as i32 + di) as usize;
            let to_j = (cur_j as i32 + dj) as usize;

            let point = self.mtx.get(to_i)?.get(to_j)?;

            Some((point, to_i, to_j))
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

    pub fn start(&self) -> Option<(usize, usize)> {
        let mut start: Option<(usize, usize)> = None;

        'out: for (i_, row_) in self.mtx.iter().enumerate() {
            for (j_, point) in row_.iter().enumerate() {
                if point == &Point::Start {
                    start = Some((i_, j_));
                    break 'out;
                }
            }
        }

        start
    }

    pub fn end(&self) -> Option<(usize, usize)> {
        let mut end: Option<(usize, usize)> = None;

        'out: for (i_, row_) in self.mtx.iter().enumerate() {
            for (j_, point) in row_.iter().enumerate() {
                if point == &Point::End {
                    end = Some((i_, j_));
                    break 'out;
                }
            }
        }

        end
    }
}
