use crate::graph::{Graph, GraphWrapper, Point, PointCoord};

pub mod bfs;
pub mod dijkstra;
pub mod heuristic;

pub use bfs::Bfs;
pub use dijkstra::Dijkstra;
pub use heuristic::Heuristic;

pub trait PathFinder {
    fn new(graph: Graph) -> Box<dyn PathFinder>
    where
        Self: Sized;

    fn tick(&mut self) -> bool;

    fn reset(&mut self);

    fn graph_wrapper<'a>(&'a self) -> &'a GraphWrapper;

    fn graph_wrapper_mut<'a>(&'a mut self) -> &'a mut GraphWrapper;

    fn graph<'a>(&'a self) -> &'a Graph {
        self.graph_wrapper().graph()
    }

    fn is_completed(&self) -> bool {
        self.graph_wrapper().completed
    }

    fn point_at<'a>(&'a self, point_coord: &PointCoord) -> &'a Point {
        self.graph_wrapper().point_at(point_coord)
    }

    fn reset_with(&mut self, graph: Graph) {
        *self.graph_wrapper_mut() = GraphWrapper::new(graph);

        self.reset();
    }
}
