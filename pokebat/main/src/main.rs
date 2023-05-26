use cli::args::Cli;
use pokemon::grid::BattleGrid;

fn main() {
    let args = Cli::get_args();
    let mut grid = BattleGrid::new(args.width, args.height, args.frame_amount);
    grid.simulate();
}
