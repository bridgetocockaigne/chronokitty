use chronokitty::storage::{Error as StorageError, Storage, StorageBuilder};
use chronokitty::task::Task;
use cucumber::{given, then, when, World};

#[derive(Debug, Default, World)]
pub struct StorageWorld {
    task: Task,
    storage: Storage,
}

#[allow(dead_code)]
pub(crate) async fn run() {
    StorageWorld::cucumber()
        .fail_on_skipped()
        .run("tests/features/storage.feature")
        .await;
}

#[given(expr = "a task with name {string}")]
async fn given_a_task(w: &mut StorageWorld, name: String) {
    w.task = Task::new(name);
}

#[given(expr = "a storage")]
async fn given_a_storage(w: &mut StorageWorld) -> Result<(), StorageError> {
    w.storage = StorageBuilder::default().build().await?;
    Ok(())
}

#[when(expr = "the tracker stores a task in storage")]
async fn stores_a_task(w: &mut StorageWorld) -> Result<(), StorageError> {
    let task = w.storage.store(&w.task).await?;
    w.task = task;

    Ok(())
}

#[then(expr = "the task gets an id")]
async fn task_have_an_id(w: &mut StorageWorld) {
    assert!(w.task.id.is_some());
}