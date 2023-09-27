mod users;

use crate::ports::database::Database;

#[derive(Clone, Default)]
pub struct App<DB>
where
    DB: Database,
{
    database: DB,
}

impl<DB> App<DB>
where
    DB: Database,
{
    pub fn with(database: DB) -> Self {
        Self { database }
    }
}

#[cfg(test)]
use crate::adapters::mocks::database::MockDatabase;

#[cfg(test)]
#[allow(unused)]
type TestApp = App<MockDatabase>;
