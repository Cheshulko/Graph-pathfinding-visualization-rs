use std::collections::VecDeque;

use crate::graph::{Graph, Point};

use super::{GraphWrapper, PathFinder};

pub struct Bfs {
    graph_wrapper: GraphWrapper,

    frontier: VecDeque<(u32, (usize, usize))>,
}

impl PathFinder for Bfs {
    fn new(graph: Graph) -> Box<dyn PathFinder> {
        let graph_wrapper = GraphWrapper::new(graph);
        let frontier = VecDeque::from_iter([(0, graph_wrapper.start)]);

        println!("[I] Bfs");

        Box::new(Self {
            graph_wrapper,
            frontier,
        })
    }

    fn tick(&mut self) -> bool {
        let mut result = false;

        if let Some((length, (cur_i, cur_j))) = self.frontier.pop_front() {
            // Mark current as seen
            if cur_i == self.graph_wrapper.start.0 && cur_j == self.graph_wrapper.start.1 {
            } else {
                self.graph_wrapper.graph.mtx[cur_i][cur_j] = Point::Seen {
                    initial_point: Box::new(self.graph_wrapper.graph.mtx[cur_i][cur_j].clone()),
                };
            }

            // Bfs doest support lengths(weights)
            let to_length = length + 1;

            for (to_point, to_i, to_j) in self.graph_wrapper.graph.neighbors(cur_i, cur_j) {
                match to_point {
                    &Point::Free if self.graph_wrapper.came_from[to_i][to_j] == None => {
                        self.graph_wrapper.came_from[to_i][to_j] =
                            Some(((cur_i, cur_j), to_length));
                        self.frontier.push_back((to_length, (to_i, to_j)));
                    }
                    &Point::End => {
                        self.graph_wrapper.came_from[to_i][to_j] =
                            Some(((cur_i, cur_j), to_length));
                        result = true;
                    }
                    _ => {}
                }
            }
        };

        if result {
            self.graph_wrapper.build_path();
            self.graph_wrapper.completed = true;
        };

        result
    }

    fn point_mut<'a>(&'a mut self, i: usize, j: usize) -> &'a mut Point {
        self.graph_wrapper.point_mut(i, j)
    }

    fn reset(&mut self) {
        self.graph_wrapper.reset();

        let frontier = VecDeque::from_iter([(0, self.graph_wrapper.start)]);
        self.frontier = frontier;
    }

    fn reset_with(&mut self, graph: Graph) {
        self.graph_wrapper = GraphWrapper::new(graph);

        self.reset();
    }

    fn point<'a>(&'a self, i: usize, j: usize) -> &'a Point {
        self.graph_wrapper.point(i, j)
    }

    fn is_completed(&self) -> bool {
        self.graph_wrapper.completed
    }
}
