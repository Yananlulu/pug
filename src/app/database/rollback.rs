use clap::{App, SubCommand};
use diesel::Connection as DieselConnection;

use super::super::super::{
    errors::{Error, Result},
    orm::{schema::Migration, Connection},
};

pub const COMMAND_NAME: &'static str = "database:rollback";

pub fn command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMMAND_NAME).about("Rollback database the last migration")
}

pub fn run(db: &Connection) -> Result<()> {
    db.transaction::<_, Error, _>(|| db.rollback())
}
