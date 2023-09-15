mod users;

pub use users::*;

use super::Service;

pub trait Database: Service + UserDatabase {}
