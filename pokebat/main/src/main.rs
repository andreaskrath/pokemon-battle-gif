use pokemon::args::Cli;
use pokemon::grid::BattleGrid;

fn main() {
    let args = Cli::get_args();
    let mut grid = BattleGrid::from(args);
    grid.simulate();
}
