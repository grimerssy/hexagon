mod users;

use crate::ports::database::Database;

#[derive(Clone, Default)]
pub struct App<DB>
where
    DB: Database,
{
    pub database: DB,
}

#[cfg(test)]
use crate::adapters::mocks::database::MockDatabase;

#[cfg(test)]
#[allow(unused)]
type TestApp = App<MockDatabase>;
