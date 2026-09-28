use forge_store_core::queue::{Action, Backend, JobQueue, JobState};

#[test]
fn queue_persists_and_serializes_per_application() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("jobs.sqlite3");
    let queue = JobQueue::open(&db).unwrap();
    let first = queue
        .enqueue("7zip", Backend::Compatforge, Action::Install)
        .unwrap();
    assert!(queue
        .enqueue("7zip", Backend::Compatforge, Action::Update)
        .is_err());
    let second = queue
        .enqueue("firefox", Backend::Flatpak, Action::Install)
        .unwrap();
    assert_eq!(queue.start_next().unwrap().unwrap().id, first.id);
    assert_eq!(queue.start_next().unwrap().unwrap().id, second.id);
    queue.finish(&first.id, true, Some("generation-1")).unwrap();
    drop(queue);
    let queue = JobQueue::open(&db).unwrap();
    assert_eq!(
        queue.get(&first.id).unwrap().unwrap().state,
        JobState::Succeeded
    );
    assert_eq!(
        queue.get(&second.id).unwrap().unwrap().state,
        JobState::Interrupted
    );
    queue
        .enqueue("firefox", Backend::Flatpak, Action::Update)
        .unwrap();
}

#[test]
fn queued_cancellation_prevents_execution() {
    let dir = tempfile::tempdir().unwrap();
    let queue = JobQueue::open(&dir.path().join("jobs.sqlite3")).unwrap();
    let job = queue
        .enqueue("7zip", Backend::Compatforge, Action::Install)
        .unwrap();
    queue.cancel(&job.id).unwrap();
    assert_eq!(
        queue.get(&job.id).unwrap().unwrap().state,
        JobState::Cancelled
    );
    assert!(queue.start_next().unwrap().is_none());
}

#[test]
fn running_cancellation_requires_worker_acknowledgement() {
    let dir = tempfile::tempdir().unwrap();
    let queue = JobQueue::open(&dir.path().join("jobs.sqlite3")).unwrap();
    let job = queue
        .enqueue("7zip", Backend::Compatforge, Action::Install)
        .unwrap();
    queue.start_next().unwrap();
    queue.cancel(&job.id).unwrap();
    assert_eq!(
        queue.get(&job.id).unwrap().unwrap().state,
        JobState::Cancelling
    );
    assert!(queue.finish(&job.id, true, Some("false-success")).is_err());
    queue.acknowledge_cancel(&job.id).unwrap();
    assert_eq!(
        queue.get(&job.id).unwrap().unwrap().state,
        JobState::Cancelled
    );
}

#[test]
fn failed_job_can_be_retried_without_reusing_identity() {
    let dir = tempfile::tempdir().unwrap();
    let queue = JobQueue::open(&dir.path().join("jobs.sqlite3")).unwrap();
    let job = queue
        .enqueue("7zip", Backend::Compatforge, Action::Install)
        .unwrap();
    queue.start_next().unwrap();
    queue
        .finish(&job.id, false, Some("installer rejected"))
        .unwrap();
    let retry = queue.retry(&job.id).unwrap();
    assert_ne!(job.id, retry.id);
    assert_eq!(retry.state, JobState::Queued);
}

#[test]
fn provider_success_after_cancel_request_is_reported_as_success() {
    let dir = tempfile::tempdir().unwrap();
    let queue = JobQueue::open(&dir.path().join("jobs.sqlite3")).unwrap();
    let job = queue
        .enqueue("7zip", Backend::Compatforge, Action::Install)
        .unwrap();
    queue.start_next().unwrap();
    queue.cancel(&job.id).unwrap();
    queue
        .finish_after_cancel(
            &job.id,
            true,
            Some("provider completed before cancellation"),
        )
        .unwrap();
    assert_eq!(
        queue.get(&job.id).unwrap().unwrap().state,
        JobState::Succeeded
    );
}

#[test]
fn recent_jobs_are_bounded_and_report_older_history() {
    let dir = tempfile::tempdir().unwrap();
    let queue = JobQueue::open(&dir.path().join("jobs.sqlite3")).unwrap();
    for index in 0..70 {
        let job = queue
            .enqueue(&format!("app{index}"), Backend::Flatpak, Action::Install)
            .unwrap();
        queue.start_next().unwrap();
        queue.finish(&job.id, true, None).unwrap();
    }
    let (recent, has_more) = queue.list_recent(64).unwrap();
    assert_eq!(recent.len(), 64);
    assert!(has_more);
    assert_eq!(recent.first().unwrap().app_id, "app6");
    assert_eq!(recent.last().unwrap().app_id, "app69");
}
