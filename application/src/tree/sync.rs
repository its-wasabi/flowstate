impl super::Tree {
    pub(super) fn generate_sync_message(
        &self,
        local_sync_state: &mut automerge::sync::State,
    ) -> Option<Vec<u8>> {
        use automerge::sync::SyncDoc;
        self.document
            .generate_sync_message(local_sync_state)
            .map(automerge::sync::Message::encode)
    }

    pub(super) fn receive_sync_message(
        &mut self,
        local_sync_state: &mut automerge::sync::State,
        bytes: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        use automerge::sync::SyncDoc;
        let msg = automerge::sync::Message::decode(bytes)?;
        self.document.receive_sync_message(local_sync_state, msg)?;
        Ok(())
    }
}
