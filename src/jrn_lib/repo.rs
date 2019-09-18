use super::entry::JrnEntry;

pub struct JrnRepo {
    /// entries sorted by creation time
    entries: Vec<JrnEntry>
}

