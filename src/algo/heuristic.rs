use std::{cmp::Reverse, collections::BinaryHeap};

use crate::graph::{GraphWrapper, Point, PointCoord};

use super::PathFinder;

pub struct Heuristic {
    graph_wrapper: GraphWrapper,

    priority_queue: BinaryHeap<(Reverse<u32>, Reverse<u32>, PointCoord)>,
}

impl Heuristic {
    // Manhattan distance on a square grid
    pub fn heuristic(&self, a: &PointCoord, b: &PointCoord) -> u32 {
        let x_max = a.x.max(b.x);
        let x_min = a.x.min(b.x);

        let y_max = a.y.min(b.y);
        let y_min = a.y.min(b.y);

        return (x_max - x_min + y_max - y_min) as u32;
    }
}

impl PathFinder for Heuristic {
    fn new(graph: crate::graph::Graph) -> Box<dyn PathFinder>
    where
        Self: Sized,
    {
        let graph_wrapper = GraphWrapper::new(graph);
        let priority_queue =
            BinaryHeap::from_iter([(Reverse(0), Reverse(0), graph_wrapper.start_coord().clone())]);

        println!("[I] Heuristic");

        Box::new(Self {
            graph_wrapper,
            priority_queue,
        })
    }

    fn tick(&mut self) -> bool {
        let mut result = false;

        while let Some((Reverse(_), Reverse(length), cur)) = self.priority_queue.pop() {
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
                        _ => return None,
                    };

                    match self.graph_wrapper.came_from[to.y][to.x] {
                        Some((_, length_best)) if length_best <= length_to => None,
                        _ => Some((to, cur.clone(), length_to)),
                    }
                })
                .collect::<Vec<_>>();

            for (to, cur, length_to) in reached_points.into_iter() {
                let heuristic_length_to = self.heuristic(&to, &self.graph().end().unwrap());

                self.priority_queue.push((
                    Reverse(heuristic_length_to),
                    Reverse(length_to),
                    to.clone(),
                ));

                self.graph_wrapper.came_from[to.y][to.x] = Some((cur, length_to));
            }

            break;
        }

        if result {
            self.graph_wrapper.build_path();
            self.graph_wrapper.completed = true;
        };

        result
    }

    fn reset(&mut self) {
        self.graph_wrapper.reset();

        let priority_queue = BinaryHeap::from_iter([(
            Reverse(0),
            Reverse(0),
            self.graph_wrapper.start_coord().clone(),
        )]);
        self.priority_queue = priority_queue;
    }

    fn graph_wrapper<'a>(&'a self) -> &'a GraphWrapper {
        &self.graph_wrapper
    }

    fn graph_wrapper_mut<'a>(&'a mut self) -> &'a mut GraphWrapper {
        &mut self.graph_wrapper
    }
}
