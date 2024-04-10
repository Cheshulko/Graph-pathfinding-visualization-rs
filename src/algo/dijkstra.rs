use std::{cmp::Reverse, collections::BinaryHeap};

use crate::graph::{GraphWrapper, PointCoord};

use super::{Graph, PathFinder, Point};

pub struct Dijkstra {
    graph_wrapper: GraphWrapper,

    priority_queue: BinaryHeap<(Reverse<u32>, PointCoord)>,
}

impl PathFinder for Dijkstra {
    fn new(graph: Graph) -> Box<dyn PathFinder> {
        let graph_wrapper = GraphWrapper::new(graph);
        let priority_queue =
            BinaryHeap::from_iter([(Reverse(0), graph_wrapper.start_coord().clone())]);

        println!("[I] Dijkstra");

        Box::new(Self {
            graph_wrapper,
            priority_queue,
        })
    }

    fn step(&mut self) {
        let mut result = false;

        while let Some((Reverse(length), cur)) = self.priority_queue.pop() {
            // Skip `worse` points
            if let Some((_, length_best)) = self.graph_wrapper.came_from[cur.y][cur.x] {
                if length_best < length {
                    continue;
                }
            }

            // Found `end`
            if &cur == self.graph_wrapper.end_coord() {
                result = true;
                break;
            }

            // Mark current as seen, not mark start
            if &cur == self.graph_wrapper.start_coord() {
            } else {
                self.graph_wrapper.seen_for_point(&cur);
            }

            let reached_points = self
                .graph_wrapper
                .graph()
                .neighbors(&cur)
                .filter_map(|(to_point, to)| {
                    let length_to = match to_point {
                        Point::Free => length + 1,
                        Point::End => length + 1,
                        Point::Obstacle {
                            length: point_length,
                        } => {
                            assert!(point_length < &4);
                            length + (point_length + 1) * Graph::OBSTACLE_DIFFICULTY_K
                        }
                        _ => return None,
                    };

                    match self.graph_wrapper.came_from[to.y][to.x] {
                        Some((_, length_best)) if length_best <= length_to => None,
                        _ => Some((to, cur.clone(), length_to)),
                    }
                })
                .collect::<Vec<_>>();

            for (to, cur, length_to) in reached_points.into_iter() {
                self.priority_queue.push((Reverse(length_to), to.clone()));
                self.graph_wrapper.came_from[to.y][to.x] = Some((cur, length_to));
            }

            break;
        }

        if result {
            self.graph_wrapper.completed = true;
        };
    }

    fn reset(&mut self) {
        self.graph_wrapper.reset();

        let priority_queue =
            BinaryHeap::from_iter([(Reverse(0), self.graph_wrapper.start_coord().clone())]);
        self.priority_queue = priority_queue;
    }

    fn graph_wrapper<'a>(&'a self) -> &'a GraphWrapper {
        &self.graph_wrapper
    }

    fn graph_wrapper_mut<'a>(&'a mut self) -> &'a mut GraphWrapper {
        &mut self.graph_wrapper
    }
}
