use cucumber::{World, given, then, when};
use horfimbor_eventsource::{Dto, State};
use mono_shared::command::Delay;
use mono_shared::command::MonoCommand::{Add, Delayed, Finalize, Reset};
use mono_shared::error::MonoError;
use mono_shared::event::MonoEvent;
use mono_state::MonoState;

#[derive(World, Debug, Default)]
pub struct MonoWorld {
    model: MonoState,
    err: Option<MonoError>,
}

#[given(regex = r"^a mono$")]
fn a_mono(world: &mut MonoWorld) {
    world.model = MonoState::default();
}

#[when(regex = r"^i try to add (\d+)")]
fn add_to_mono(world: &mut MonoWorld, nb: usize) {
    play_result(world, world.model.try_command(Add(nb)));
}

#[when(regex = r"^i try to delay (\d+) by (\d+) secs")]
fn delay_mono(world: &mut MonoWorld, nb: usize, delay: usize) {
    play_result(
        world,
        world
            .model
            .try_command(Delayed(Delay { delay, to_add: nb })),
    );
}

#[when(regex = r"^i try to reset it$")]
fn reset_mono(world: &mut MonoWorld) {
    play_result(world, world.model.try_command(Reset));
}

#[when(regex = r"^i wait (\d+) seconds$")]
fn wait_mono(world: &mut MonoWorld, nb: u64) {
    world.model.time_pass(nb);
}
#[when(regex = r"^i try to finalize the delay (\d+)$")]
fn finalize_mono(world: &mut MonoWorld, nb: usize) {
    let id = world.model.get_id(nb);
    play_result(world, world.model.try_command(Finalize(id)));
}

#[then(regex = r"^it got a value of (\d+)$")]
fn result(world: &mut MonoWorld, nb: usize) {
    assert_eq!(nb, world.model.get_value());
}

#[then(regex = r"^it got an error$")]
fn error(world: &mut MonoWorld) {
    assert!(world.err.is_some());
}

#[then(regex = r"^remaining delay is (\d+)$")]
fn remaining(world: &mut MonoWorld, nb: usize) {
    assert_eq!(nb, world.model.delayed().len());
}

fn play_result(world: &mut MonoWorld, events: Result<Vec<MonoEvent>, MonoError>) {
    match events {
        Ok(list) => {
            for e in list {
                world.model.play_event(&e);
            }
        }
        Err(e) => world.err = Some(e),
    }
}

#[tokio::main]
async fn main() {
    MonoWorld::run("tests/book").await;
}
