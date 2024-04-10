use crate::graph::{Graph, Point, PointCoord};

pub struct GraphWrapper {
    /* fuck */ graph: Graph,
    start: PointCoord,
    end: PointCoord,

    pub came_from: Vec<Vec<Option<(PointCoord, u32)>>>,
    pub completed: bool,
}

impl GraphWrapper {
    pub fn new(graph: Graph) -> Self {
        let start = graph.start().expect("No start point");
        let end = graph.end().expect("No end point");

        let n = graph.n();
        let m = graph.m();

        let mut came_from = vec![vec![None; m]; n];
        came_from[start.y][start.x] = Some((start.clone(), 0));

        GraphWrapper {
            graph,
            start,
            end,
            came_from,
            completed: false,
        }
    }

    pub fn reset(&mut self) {
        self.graph.reset();
        self.completed = false;

        let n = self.graph.n();
        let m = self.graph.m();
        self.came_from = vec![vec![None; m]; n];
        self.came_from[self.start.y][self.start.x] = Some((self.start.clone(), 0));
    }

    pub fn build_path(&mut self) {
        let mut cur = self.end.clone();

        while let Some((cur_, _length)) = &self.came_from[cur.y][cur.x] {
            if &self.start == cur_ {
                break;
            }

            // So ok, we can modify graph's mtx directlly cus the same module
            self.graph.mtx[cur_.y][cur_.x] = Point::Path {
                initial_point: Box::new(match self.graph.point_at(&cur_) {
                    &Point::Seen { ref initial_point } => *initial_point.clone(),
                    _ => unreachable!(),
                }),
            };

            cur = cur_.clone();
        }
    }

    pub fn seen_for_point(&mut self, point_coord: &PointCoord) {
        let point = self.point_at_mut(point_coord);

        *point = Point::Seen {
            initial_point: Box::new(point.clone()),
        };
    }

    pub fn point_at<'a>(&'a self, point_coord: &PointCoord) -> &'a Point {
        self.graph.point_at(point_coord)
    }

    pub fn start_coord<'a>(&'a self) -> &'a PointCoord {
        &self.start
    }

    pub fn end_coord<'a>(&'a self) -> &'a PointCoord {
        &self.end
    }

    pub fn graph<'a>(&'a self) -> &'a Graph {
        &self.graph
    }

    fn point_at_mut<'a>(&'a mut self, point_coord: &PointCoord) -> &'a mut Point {
        self.graph.point_at_mut(point_coord)
    }
}
