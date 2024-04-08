use crate::graph::{Graph, Point};

pub mod bfs;
pub mod dijkstra;

pub use bfs::Bfs;
pub use dijkstra::Dijkstra;

pub struct GraphWrapper {
    graph: Graph,

    start: (usize, usize),
    end: (usize, usize),

    came_from: Vec<Vec<Option<((usize, usize), u32)>>>,

    completed: bool,
}

impl GraphWrapper {
    pub fn new(graph: Graph) -> Self {
        let start = graph.start().expect("No start point");
        let end = graph.end().expect("No end point");

        let n = graph.n();
        let m = graph.m();

        let mut came_from = vec![vec![None; m]; n];
        came_from[start.0][start.1] = Some(((start.0, start.1), 0));

        GraphWrapper {
            graph,
            start,
            end,
            came_from,
            completed: false,
        }
    }

    pub fn reset(&mut self) {
        for row in &mut self.graph.mtx {
            for point in row {
                match point {
                    Point::Path { initial_point } => *point = *initial_point.clone(),
                    Point::Seen { initial_point } => *point = *initial_point.clone(),
                    _ => {}
                }
            }
        }

        self.completed = false;

        let n = self.graph.n();
        let m = self.graph.m();
        self.came_from = vec![vec![None; m]; n];
        self.came_from[self.start.0][self.start.1] = Some(((self.start.0, self.start.1), 0));
    }

    pub fn build_path(&mut self) {
        let mut cur_i = self.end.0;
        let mut cur_j = self.end.1;

        while let Some(((cur_i_, cur_j_), _length)) = self.came_from[cur_i][cur_j] {
            if cur_i_ == self.start.0 && cur_j_ == self.start.1 {
                break;
            }

            self.graph.mtx[cur_i_][cur_j_] = Point::Path {
                initial_point: Box::new(match &self.graph.mtx[cur_i_][cur_j_] {
                    &Point::Seen { ref initial_point } => *initial_point.clone(),
                    _ => unreachable!(),
                }),
            };

            cur_i = cur_i_;
            cur_j = cur_j_;
        }
    }

    pub fn point_mut<'a>(&'a mut self, i: usize, j: usize) -> &'a mut Point {
        &mut self.graph.mtx[i][j]
    }

    pub fn point<'a>(&'a self, i: usize, j: usize) -> &'a Point {
        &self.graph.mtx[i][j]
    }
}

pub trait PathFinder {
    fn new(graph: Graph) -> Box<dyn PathFinder>
    where
        Self: Sized;

    fn is_completed(&self) -> bool;

    fn point<'a>(&'a self, i: usize, j: usize) -> &'a Point;

    fn point_mut<'a>(&'a mut self, i: usize, j: usize) -> &'a mut Point;

    fn tick(&mut self) -> bool;

    fn reset(&mut self);

    fn reset_with(&mut self, graph: Graph);
}
