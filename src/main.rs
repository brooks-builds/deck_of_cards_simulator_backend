use async_std::task;
use deck_of_cards_simulator_backend::run;
use eyre::Result;

fn main() -> Result<()> {
    task::block_on(run())
}
