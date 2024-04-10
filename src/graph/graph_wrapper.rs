use crate::graph::{Graph, Point, PointCoord};

pub struct GraphWrapper {
    graph: Graph,

    start_coord: PointCoord,
    end_coord: PointCoord,

    /*hz*/ pub completed: bool,

    pub seen_points: u32,
    pub came_from: Vec<Vec<Option<(PointCoord, u32)>>>,
}

pub struct GraphPath<'a> {
    from_coord: &'a PointCoord,
    to_coord: &'a PointCoord,

    seen_points: u32,
    length: u32,
}

impl std::fmt::Display for GraphPath<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path\n")?;
        write!(f, "\tFrom:\t\t{}\n", self.from_coord)?;
        write!(f, "\tTo:\t\t{}\n", self.to_coord)?;
        write!(f, "\tSeen points:\t{}\n", self.seen_points)?;
        write!(f, "\tLength:\t\t{}\n", self.length)
    }
}

impl GraphWrapper {
    pub fn new(graph: Graph) -> Self {
        let start_coord = graph.start().expect("No start point");
        let end_coord = graph.end().expect("No end point");

        let n = graph.n();
        let m = graph.m();

        let mut came_from = vec![vec![None; m]; n];
        came_from[start_coord.y][start_coord.x] = Some((start_coord.clone(), 0));

        GraphWrapper {
            graph,
            start_coord,
            end_coord,
            came_from,
            seen_points: 0,
            completed: false,
        }
    }

    pub fn reset(&mut self) {
        self.graph.reset();
        self.completed = false;
        self.seen_points = 0;

        let n = self.graph.n();
        let m = self.graph.m();
        self.came_from = vec![vec![None; m]; n];
        self.came_from[self.start_coord.y][self.start_coord.x] =
            Some((self.start_coord.clone(), 0));
    }

    // TODO: Do not build path if it is already built
    pub fn build_path<'a>(&'a mut self) -> GraphPath<'a> {
        let mut cur = self.end_coord.clone();

        let (_, length) = &self.came_from[cur.y][cur.x]
            .clone()
            .expect("Something went wrong. End point is not reached?");

        while let Some((cur_, _length)) = &self.came_from[cur.y][cur.x] {
            if &self.start_coord == cur_ {
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

        GraphPath {
            from_coord: &self.start_coord,
            to_coord: &self.end_coord,

            seen_points: self.seen_points,
            length: *length,
        }
    }

    pub fn seen_for_point(&mut self, point_coord: &PointCoord) {
        self.seen_points += 1;

        let point = self.point_at_mut(point_coord);

        *point = Point::Seen {
            initial_point: Box::new(point.clone()),
        };
    }

    pub fn point_at<'a>(&'a self, point_coord: &PointCoord) -> &'a Point {
        self.graph.point_at(point_coord)
    }

    pub fn start_coord<'a>(&'a self) -> &'a PointCoord {
        &self.start_coord
    }

    pub fn end_coord<'a>(&'a self) -> &'a PointCoord {
        &self.end_coord
    }

    pub fn graph<'a>(&'a self) -> &'a Graph {
        &self.graph
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    fn point_at_mut<'a>(&'a mut self, point_coord: &PointCoord) -> &'a mut Point {
        self.graph.point_at_mut(point_coord)
    }
}
