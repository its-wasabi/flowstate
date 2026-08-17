pub(super) trait TransactionExt {
    fn commit_with_time(self) -> (Option<automerge::ChangeHash>, automerge::PatchLog);
}

impl TransactionExt for automerge::transaction::Transaction<'_> {
    fn commit_with_time(self) -> (Option<automerge::ChangeHash>, automerge::PatchLog) {
        self.commit_with(
            automerge::transaction::CommitOptions::default()
                .with_time(chrono::Utc::now().timestamp_millis()),
        )
    }
}
