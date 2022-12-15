use crate::{
    database::Database,
    state::in_memory::transaction::MemoryTransactionView,
};
use fuel_core_interfaces::db::{
    Error,
    Transactional,
};
use std::{
    fmt::Debug,
    ops::{
        Deref,
        DerefMut,
    },
    sync::Arc,
};

#[derive(Clone, Debug)]
pub struct DatabaseTransaction {
    // The primary datastores
    changes: Arc<MemoryTransactionView>,
    // The inner db impl using these stores
    database: Database,
}

impl AsRef<Database> for DatabaseTransaction {
    fn as_ref(&self) -> &Database {
        &self.database
    }
}

impl AsMut<Database> for DatabaseTransaction {
    fn as_mut(&mut self) -> &mut Database {
        self.deref_mut()
    }
}

impl Deref for DatabaseTransaction {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.database
    }
}

impl DerefMut for DatabaseTransaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.database
    }
}

impl Default for DatabaseTransaction {
    fn default() -> Self {
        Database::default().transaction()
    }
}

impl Transactional for DatabaseTransaction {
    fn commit(self) -> crate::state::Result<()> {
        // TODO: should commit be fallible if this api is meant to be atomic?
        self.changes.commit()
    }

    fn commit_box(self: Box<Self>) -> Result<(), Error> {
        self.changes.commit()
    }
}

impl fuel_core_interfaces::db::DatabaseTransaction<Database> for DatabaseTransaction {
    fn database(&self) -> &Database {
        &self.database
    }

    fn database_mut(&mut self) -> &mut Database {
        &mut self.database
    }
}

impl From<&Database> for DatabaseTransaction {
    fn from(source: &Database) -> Self {
        let data = Arc::new(MemoryTransactionView::new(source.data.clone()));
        Self {
            changes: data.clone(),
            database: Database {
                data,
                _drop: Default::default(),
            },
        }
    }
}