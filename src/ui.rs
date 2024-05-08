use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use anyhow::Context;

use crate::algo::PathFinder;
use crate::graph::{Generation, Point, PointCoord};
use crate::{algo, graph};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct World {
    row_height: u32,
    column_width: u32,

    algo: Box<dyn PathFinder>,
}

impl World {
    const SEEN_CELL_BORDER: u32 = 10;

    const GRID_COLOR: &[u8] = &[0x5e, 0x48, 0xe8, 0xff];
    const BACKGROUND_COLOR: &[u8] = &[0x18, 0x18, 0x18, 0xff];
    const START_COLOR: &[u8] = &[0xff, 0x00, 0x00, 0xff];
    const END_COLOR: &[u8] = &[0xd7, 0x42, 0xf5, 0xff];
    const PATH_COLOR: &[u8] = &[0xff, 0xb0, 0x00, 0xff];
    const OBSTACLE_COLOR: [&[u8]; 4] = [
        &[0x60, 0xbf, 0x74, 0xff],
        &[0x40, 0xbf, 0x74, 0xbf],
        &[0x20, 0xbf, 0x74, 0x80],
        &[0x00, 0xbf, 0x74, 0x40],
    ];
    const SEEN_COLOR: &[u8] = &[0xff, 0xff, 0x91, 0xff];
}

impl World {
    fn new() -> anyhow::Result<Self> {
        let graph = graph::Graph::generate_graph(Generation::Predefined1);

        let row_height = HEIGHT / graph.n() as u32;
        let column_width = WIDTH / graph.m() as u32;

        let algo = algo::Dijkstra::new(graph);

        Ok(Self {
            row_height,
            column_width,
            algo,
        })
    }

    fn color_by_point<'a>(&'a self, point: &'a Point) -> &'a [u8] {
        return match point {
            &Point::Start => Self::START_COLOR,
            &Point::End => Self::END_COLOR,
            &Point::Free => Self::BACKGROUND_COLOR,
            &Point::Obstacle { length } => {
                assert!(length < 4);
                Self::OBSTACLE_COLOR[length as usize]
            }
            &Point::Path { .. } => Self::PATH_COLOR,
            &Point::Seen { .. } => Self::SEEN_COLOR,
        };
    }

    fn initial_color_by_point<'a>(&'a self, point: &'a Point) -> Option<&'a [u8]> {
        return match point {
            &Point::Path { ref initial_point } => Some(self.color_by_point(&initial_point)),
            &Point::Seen { ref initial_point } => Some(self.color_by_point(&initial_point)),
            _ => None,
        };
    }

    fn cell_color<'a>(&'a self, pixel_x: u32, pixel_y: u32) -> &'a [u8] {
        let point_i = pixel_y / self.row_height;
        let point_j = pixel_x / self.column_width;

        let start_y_1 = self.row_height * point_i as u32;
        let start_y_2 = self.row_height * (point_i + 1) as u32;

        let start_x_1 = self.column_width * point_j as u32;
        let start_x_2 = self.column_width * (point_j + 1) as u32;

        let point = self.algo.point_at(&PointCoord {
            y: point_i as usize,
            x: point_j as usize,
        });

        if (pixel_y >= start_y_1 && pixel_y <= start_y_2)
            && (pixel_x >= start_x_1 && pixel_x <= start_x_2)
        {
            if let Some(initial_color) = self.initial_color_by_point(point) {
                if !((pixel_y >= start_y_1 + Self::SEEN_CELL_BORDER
                    && pixel_y + Self::SEEN_CELL_BORDER <= start_y_2)
                    && (pixel_x >= start_x_1 + Self::SEEN_CELL_BORDER
                        && pixel_x + Self::SEEN_CELL_BORDER <= start_x_2))
                {
                    return initial_color;
                }
            }

            return self.color_by_point(point);
        }

        Self::BACKGROUND_COLOR
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let pixel_x = i as u32 % WIDTH;
            let pixel_y = i as u32 / WIDTH;

            let mut rgba = self.cell_color(pixel_x, pixel_y);

            if pixel_y % self.row_height == 0 || pixel_x % self.column_width == 0 {
                rgba = Self::GRID_COLOR
            }

            pixel.copy_from_slice(&rgba);
        }
    }

    fn update(&mut self) {}
}

pub fn start_ui() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Graph pathfinding visualization")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .with_context(|| "Could not create window")?
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut world = World::new().with_context(|| "Could not create a world")?;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if let Err(_err) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            // Resize
            if let Some(size) = input.window_resized() {
                if let Err(_err) = pixels.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Keys
            /*
                `s` - make algorithm's step
                `r` - reset graph to initial state

                `d` - set dijksta's algorithm
                `b` - set bfs algorithm
                `h` - set heuristic algorithm
                `a` - set a-star algorithm

                `1` - set 1' predefined graph
                `2` - set 2' predefined graph
                `-` - generate ramdom graph
            */
            if input.key_pressed(VirtualKeyCode::Q) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed_os(VirtualKeyCode::S) {
                if !world.algo.is_completed() {
                    world.algo.step();

                    if world.algo.is_completed() {
                        if let Some(path) = world.algo.build_path() {
                            println!("[I] {path}");
                        } else {
                            println!("[I] Completed. Path is not found");
                        }
                    }
                }
            }

            if input.key_pressed_os(VirtualKeyCode::R) {
                world.algo.reset()
            }

            if input.key_pressed_os(VirtualKeyCode::D) {
                let graph = world.algo.graph().clone();
                world.algo = algo::Dijkstra::new(graph);
                world.algo.reset();
            }

            if input.key_pressed_os(VirtualKeyCode::B) {
                let graph = world.algo.graph().clone();
                world.algo = algo::Bfs::new(graph);
                world.algo.reset();
            }

            if input.key_pressed_os(VirtualKeyCode::H) {
                let graph = world.algo.graph().clone();
                world.algo = algo::Heuristic::new(graph);
                world.algo.reset();
            }

            if input.key_pressed_os(VirtualKeyCode::A) {
                let graph = world.algo.graph().clone();
                world.algo = algo::AStar::new(graph);
                world.algo.reset();
            }

            if input.key_pressed_os(VirtualKeyCode::Key1) {
                let graph = graph::Graph::generate_graph(Generation::Predefined1);
                world.algo.reset_with(graph);
            }

            if input.key_pressed_os(VirtualKeyCode::Key2) {
                let graph = graph::Graph::generate_graph(Generation::Predefined2);
                world.algo.reset_with(graph);
            }

            if input.key_pressed_os(VirtualKeyCode::Minus) {
                let graph = graph::Graph::generate_graph(Generation::Random);
                world.algo.reset_with(graph);
            }

            world.update();
            window.request_redraw();
        }
    })
}
