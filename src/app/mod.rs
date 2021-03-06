pub mod database;
pub mod generate;

use std::result::Result as StdResult;

use clap::{self, SubCommand};
use log4rs;
use rust_sodium;
use serde::{de::DeserializeOwned, ser::Serialize};

use super::{
    env::Config,
    errors::{Error, Result},
    orm::{schema::New as Migration, PooledConnection as DbPooledConnection},
    parser,
};

pub trait Server {
    type Config: Serialize + DeserializeOwned + Default + Into<Config>;
    type Error: From<Error>;
    fn launch(&self, &Self::Config) -> StdResult<(), Self::Error>;
    fn migrations(&self) -> Vec<Migration>;
}

pub struct App<'a, 'b> {
    app: clap::App<'a, 'b>,
}

impl<'a, 'b> App<'a, 'b> {
    pub fn new(
        name: &'a str,
        version: &'a str,
        author: Option<&'a str>,
        about: Option<&'a str>,
        banner: Option<&'a str>,
        homepage: Option<&'a str>,
    ) -> Result<Self> {
        log4rs::init_file("log4rs.yml", Default::default())?;
        if let Err(_) = rust_sodium::init() {
            return Err("sodium init fail".into());
        }

        let mut app = clap::App::new(name).version(version);
        if let Some(v) = author {
            app = app.author(v);
        }
        if let Some(v) = about {
            app = app.about(v);
        }
        if let Some(v) = banner {
            app = app.before_help(v);
        }
        if let Some(v) = homepage {
            app = app.after_help(v);
        }
        Ok(Self { app: app })
    }

    pub fn launch<S, C, E>(self, server: &S) -> StdResult<(), E>
    where
        S: Server<Config = C, Error = E>,
        C: Serialize + DeserializeOwned + Default + Into<Config>,
        E: From<Error>,
    {
        let cfg = "config.toml";
        let meta = self.app.p.meta.clone();
        let matches = self
            .app
            .subcommand(generate::nginx::command())
            .subcommand(
                SubCommand::with_name(generate::config::NAME).about(&*generate::config::help(cfg)),
            )
            .subcommand(generate::systemd::command())
            .subcommand(database::migrate::command())
            .subcommand(database::rollback::command())
            .subcommand(database::status::command())
            .get_matches();

        if let Some(_) = matches.subcommand_matches(generate::config::NAME) {
            generate::config::run::<&'static str, C>(cfg)?;
            return Ok(());
        }

        info!("load configuration from {}", cfg);
        let cfg: C = parser::toml(cfg)?;
        if let Some(matches) = matches.subcommand_matches(generate::nginx::COMMAND_NAME) {
            let cfg: Config = cfg.into();
            let name = matches.value_of(generate::nginx::ARG_SERVER_NAME).unwrap();
            generate::nginx::run(
                name.to_string(),
                cfg.http.port,
                matches.is_present(generate::nginx::ARG_HTTPS),
            )?;
            return Ok(());
        }
        if let Some(matches) = matches.subcommand_matches(generate::systemd::COMMAND_NAME) {
            let name = matches
                .value_of(generate::systemd::ARG_SERVICE_NAME)
                .unwrap();
            generate::systemd::run(
                name.to_string(),
                match meta.about {
                    Some(v) => v.to_string(),
                    None => "TODO".to_string(),
                },
            )?;
            return Ok(());
        }

        if let Some(_) = matches.subcommand_matches(database::migrate::COMMAND_NAME) {
            let db = open_database(cfg)?;
            database::migrate::run(&db, &server.migrations())?;
            return Ok(());
        }
        if let Some(_) = matches.subcommand_matches(database::rollback::COMMAND_NAME) {
            let db = open_database(cfg)?;
            database::rollback::run(&db)?;
            return Ok(());
        }
        if let Some(_) = matches.subcommand_matches(database::status::COMMAND_NAME) {
            let db = open_database(cfg)?;
            database::status::run(&db)?;
            return Ok(());
        }

        server.launch(&cfg)?;
        Ok(())
    }
}

fn open_database<C: Into<Config>>(cfg: C) -> Result<DbPooledConnection> {
    let cfg: Config = cfg.into();
    let db = cfg.database()?;
    let db = db.get()?;
    Ok(db)
}
