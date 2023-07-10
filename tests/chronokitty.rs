use chronokitty::{
    storage::{Error, Storage, StorageBuilder},
    task::Task,
};
use cucumber::{given, then, when, World};

#[derive(Default, Debug, World)]
pub struct ChronoWorld {
    task: Task,
    storage: Option<Storage>,
}

#[given(expr = "a task")]
async fn create_a_task(w: &mut ChronoWorld) {
    w.task = Task::default();
}

#[given(expr = "a storage")]
async fn create_a_storage(w: &mut ChronoWorld) -> Result<(), Error> {
    let storage = StorageBuilder::default().build().await?;
    w.storage = Some(storage);
    Ok(())
}

#[when(expr = "the user stops the task")]
async fn user_stops_the_task(w: &mut ChronoWorld) {
    w.task.stop();
}

#[when(expr = "the user starts the task")]
async fn user_starts_the_task(w: &mut ChronoWorld) {
    w.task.start();
}

#[then(expr = "the task is stored")]
async fn task_is_stored(w: &mut ChronoWorld) -> Result<(), Error> {
    let storage = w.storage.as_ref().ok_or(Error::GenericError())?;
    let task = storage.save(&w.task).await?;
    assert!(task.id.is_some());
    Ok(())
}

#[then(expr = "the task as a duration bigger then 0")]
async fn task_duration_bigger_then_zero(w: &mut ChronoWorld) {
    assert!(!w.task.duration().is_zero());
}

#[tokio::main]
async fn main() {
    ChronoWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/chronokitty.feature")
        .await;
}
