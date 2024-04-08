use std::{cmp::Reverse, collections::BinaryHeap};

use super::{Graph, GraphWrapper, PathFinder, Point};

pub struct Dijkstra {
    graph_wrapper: GraphWrapper,

    priority_queue: BinaryHeap<(Reverse<u32>, (usize, usize))>,
}

impl Dijkstra {
    const OBSTACLE_DIFFICULTY_K: u32 = 6;
}

impl PathFinder for Dijkstra {
    fn new(graph: Graph) -> Box<dyn PathFinder> {
        let graph_wrapper = GraphWrapper::new(graph);
        let priority_queue = BinaryHeap::from_iter([(Reverse(0), graph_wrapper.start)]);

        println!("[I] Dijkstra");

        Box::new(Self {
            graph_wrapper,
            priority_queue,
        })
    }

    fn tick(&mut self) -> bool {
        let mut result = false;

        while let Some((Reverse(length), (cur_i, cur_j))) = self.priority_queue.pop() {
            // Skip `worse` points
            if let Some(((_, _), length_best)) = self.graph_wrapper.came_from[cur_i][cur_j] {
                if length_best < length {
                    continue;
                }
            }

            // Found `end`
            if cur_i == self.graph_wrapper.end.0 && cur_j == self.graph_wrapper.end.1 {
                result = true;
                break;
            }

            // Mark current as seen
            if cur_i == self.graph_wrapper.start.0 && cur_j == self.graph_wrapper.start.1 {
            } else {
                self.graph_wrapper.graph.mtx[cur_i][cur_j] = Point::Seen {
                    initial_point: Box::new(self.graph_wrapper.graph.mtx[cur_i][cur_j].clone()),
                };
            }

            for (to_point, to_i, to_j) in self.graph_wrapper.graph.neighbors(cur_i, cur_j) {
                let length_to = match to_point {
                    Point::Free => length + 1,
                    Point::End => length + 1,
                    Point::Obstacle {
                        length: point_length,
                    } => {
                        assert!(point_length < &4);
                        length + (point_length + 1) * Self::OBSTACLE_DIFFICULTY_K
                    }
                    _ => continue,
                };

                match self.graph_wrapper.came_from[to_i][to_j] {
                    Some(((_, _), length_best)) if length_best <= length_to => {}
                    _ => {
                        self.graph_wrapper.came_from[to_i][to_j] =
                            Some(((cur_i, cur_j), length_to));
                        self.priority_queue.push((Reverse(length_to), (to_i, to_j)));
                    }
                }
            }

            break;
        }

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

        let priority_queue = BinaryHeap::from_iter([(Reverse(0), self.graph_wrapper.start)]);
        self.priority_queue = priority_queue;
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
