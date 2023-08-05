use core::fmt;

use std::{path::PathBuf, str::FromStr};

use anyhow::Context;
use once_cell::sync::OnceCell;
use serde::{de::DeserializeOwned, Deserialize};
use strum::VariantNames;
use strum_macros::{Display, EnumString, EnumVariantNames};

use crate::{services::Service, Api};

static CONFIG: OnceCell<config::Config> = OnceCell::new();

#[derive(Clone, Copy, Debug, Display, EnumString, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
enum Environment {
    Development,
    Production,
}

#[derive(Clone, Deserialize)]
pub struct Config<API: Api, APP: Service> {
    #[serde(flatten)]
    pub api: API::Config,
    #[serde(flatten)]
    pub app: APP::Config,
}

impl<API, APP> Config<API, APP>
where
    API: Api,
    APP: Service,
{
    pub fn init() -> anyhow::Result<Self> {
        Self::deserialize_into()
    }

    fn deserialize_into<C: DeserializeOwned>() -> anyhow::Result<C> {
        CONFIG
            .get_or_try_init(Self::build)?
            .clone()
            .try_deserialize::<C>()
            .context("Failed to deserialize configuration")
    }

    fn build() -> anyhow::Result<config::Config> {
        let environment = Self::environment()?;
        let config_file = Self::config_path(environment)?;
        config::Config::builder()
            .add_source(config::File::from(config_file))
            .add_source(config::Environment::default().separator("__"))
            .build()
            .context("Failed to read configuration")
    }

    fn environment() -> anyhow::Result<Environment> {
        std::env::var("ENVIRONMENT")
            .context("ENVIRONMENT must be present")
            .map(|env| Environment::from_str(env.as_str()))?
            .context(format!("Valid options are: {:?}", Environment::VARIANTS))
            .context("Failed to determine application environment")
    }

    fn config_path(environment: Environment) -> anyhow::Result<PathBuf> {
        std::env::current_dir()
            .context("Could not determine current working directory")
            .map(|dir| dir.join("config").join(format!("{environment}.yaml")))
            .context("Failed to get the configuration file")
    }
}

// Workaround to implement `Debug` for config
// #[derive(Debug)] requires all generic parameters to also implement it
// even if they are not used directly
impl<API, APP> fmt::Debug for Config<API, APP>
where
    API: Api,
    APP: Service,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config {{ api: {:?}, app: {:?} }}", self.api, self.app)
    }
}
