use std::collections::VecDeque;

use crate::graph::{Graph, GraphWrapper, Point, PointCoord};

use super::PathFinder;

pub struct Bfs {
    graph_wrapper: GraphWrapper,

    frontier: VecDeque<(u32, PointCoord)>,
}

impl PathFinder for Bfs {
    fn new(graph: Graph) -> Box<dyn PathFinder> {
        let graph_wrapper = GraphWrapper::new(graph);
        let frontier = VecDeque::from_iter([(0, graph_wrapper.start_coord().clone())]);

        println!("[I] Bfs");

        Box::new(Self {
            graph_wrapper,
            frontier,
        })
    }

    fn step(&mut self) {
        let mut result = false;

        if let Some((length, cur)) = self.frontier.pop_front() {
            // Mark current as seen
            if &cur == self.graph_wrapper.start_coord() {
            } else {
                self.graph_wrapper.seen_for_point(&cur);
            }

            // Bfs doest support lengths(weights)
            let to_length = length + 1;

            let reached_points = self
                .graph_wrapper
                .graph()
                .neighbors(&cur)
                .filter_map(|(to_point, to)| match to_point {
                    &Point::Free if self.graph_wrapper.came_from[to.y][to.x] == None => {
                        Some((to, cur.clone(), to_length))
                    }

                    &Point::End => {
                        result = true;
                        Some((to, cur.clone(), to_length))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>();

            for (to, cur, to_length) in reached_points.into_iter() {
                self.frontier.push_back((to_length, to.clone()));
                self.graph_wrapper.came_from[to.y][to.x] = Some((cur.clone(), to_length));
            }
        };

        if result {
            self.graph_wrapper.completed = true;
        };
    }

    fn reset(&mut self) {
        self.graph_wrapper.reset();

        let frontier = VecDeque::from_iter([(0, self.graph_wrapper.start_coord().clone())]);
        self.frontier = frontier;
    }

    fn graph_wrapper<'a>(&'a self) -> &'a GraphWrapper {
        &self.graph_wrapper
    }

    fn graph_wrapper_mut<'a>(&'a mut self) -> &'a mut GraphWrapper {
        &mut self.graph_wrapper
    }
}
