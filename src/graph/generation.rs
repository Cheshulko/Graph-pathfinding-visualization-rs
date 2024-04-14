use rand::Rng;

use crate::graph::Point;

use super::Graph;

pub enum Generation {
    Predefined1,
    Predefined2,
    Random,
}

impl Graph {
    pub fn generate_graph(generation: Generation) -> Self {
        match generation {
            Generation::Predefined1 => Graph::generate_graph_predefined_1(),
            Generation::Predefined2 => Graph::generate_graph_predefined_2(),
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

    fn generate_graph_predefined_2() -> Self {
        println!("[I] generate_graph_predefined_2");

        let mut mtx = vec![vec![Point::Free; Self::M]; Self::N];

        mtx[Self::N - 2][0] = Point::Start;
        mtx[1][Self::M - 2] = Point::End;

        for i in 2..Self::N - 1 {
            mtx[i][7] = Point::Obstacle { length: 3 };
        }

        for j in 3..Self::M - 2 {
            mtx[2][j] = Point::Obstacle { length: 3 };
            mtx[Self::N - 2][j] = Point::Obstacle { length: 3 };
        }

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
}
