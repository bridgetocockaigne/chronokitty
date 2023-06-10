use std::usize;

use chrono::Duration;
use cucumber::{then, when, World};

#[derive(Debug, Default, World)]
pub struct ChronoKittyWorld {
    task: chronokitty::Task,
}

#[when(expr = "the tracker starts a new task with name {string}")]
async fn tracker_stars_a_new_task(w: &mut ChronoKittyWorld, name: String) {
    w.task = chronokitty::Task::new(name);
}

#[then(expr = "the tracked time is greater then {int}")]
async fn the_tracked_time_is_greater_then(w: &mut ChronoKittyWorld, time: i64) {
    let duration = Duration::seconds(time);

    assert!(
        w.task.elapsed_time().gt(&duration),
        "the tracked time was not greater than {}, elipsed time: {}",
        time,
        w.task.elapsed_time()
    )
}

#[tokio::main]
async fn main() {
    ChronoKittyWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/chronokitty.feature")
        .await;
}
