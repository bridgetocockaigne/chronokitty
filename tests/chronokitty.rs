use std::usize;

use chrono::Duration;
use cucumber::{given, then, when, World};

#[derive(Debug, Default, World)]
pub struct ChronoKittyWorld {
    task: chronokitty::Task,
}

#[given(expr = "a task with name {string}")]
async fn given_a_task(w: &mut ChronoKittyWorld, name: String) {
    w.task = chronokitty::Task::new(name);
}

#[when(expr = "the tracker starts the task")]
async fn the_tracker_starts_the_task(w: &mut ChronoKittyWorld) {
    w.task.start();
}
#[then(expr = "the task is started")]
async fn the_task_is_started(w: &mut ChronoKittyWorld) {
    assert!(w.task.is_started());
}
#[when(expr = "the tracker stops the task")]
async fn the_tracker_stops_the_task(w: &mut ChronoKittyWorld) {
    w.task.stop();
}

#[then(expr = "the task is stoped")]
async fn the_task_is_stoped(w: &mut ChronoKittyWorld) {
    assert!(w.task.is_stoped());
}

#[then(expr = "the task have a duration bigger then zero")]
async fn the_task_have_duration_bigger_then(w: &mut ChronoKittyWorld) {
    assert!(w.task.duration().gt(&Duration::zero()))
}

#[then(expr = "the task have a duration is zero")]
async fn the_task_have_duration_is_zero(w: &mut ChronoKittyWorld) {
    assert!(w.task.duration().eq(&Duration::zero()))
}

#[tokio::main]
async fn main() {
    ChronoKittyWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/chronokitty.feature")
        .await;
}
