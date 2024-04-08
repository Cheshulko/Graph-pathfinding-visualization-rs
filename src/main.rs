mod algo;
mod graph;
mod ui;

use ui::start_ui;

fn main() -> anyhow::Result<()> {
    start_ui()
}
