use crate::graph::{Graph, Point, PointCoord};

pub mod bfs;
pub mod dijkstra;

pub use bfs::Bfs;
pub use dijkstra::Dijkstra;

pub trait PathFinder {
    fn new(graph: Graph) -> Box<dyn PathFinder>
    where
        Self: Sized;

    fn graph<'a>(&'a self) -> &'a Graph;

    fn is_completed(&self) -> bool;

    fn point_at<'a>(&'a self, point_coord: &PointCoord) -> &'a Point;

    fn tick(&mut self) -> bool;

    fn reset(&mut self);

    fn reset_with(&mut self, graph: Graph);
}
