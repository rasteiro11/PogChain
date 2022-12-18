use std::{env, error::Error, fmt::Display, path::Path};

pub static CONFIG: EnvConfig =
    EnvConfig::new(".env").expect("Something went wrong during configuration load");

#[derive(Debug)]
struct LoadFileError(String);

impl std::error::Error for LoadFileError {}

impl Display for LoadFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait Config {
    fn get_difficulty(&self) -> u64;
    fn get_reward(&self) -> u64;
    fn get_genesis_prev_hash(&self) -> String;
}

pub struct EnvConfig {
    difficulty: u64,
    reward: u64,
    genesis_prev_hash: String,
}

impl EnvConfig {
    pub fn new(name: impl Into<String>) -> Result<EnvConfig, Box<dyn Error>> {
        let path = dotenvy::from_filename(name.into());
        if let Err(e) = path {
            return Err(Box::new(LoadFileError(
                "Something went wrong loading file".to_string(),
            )));
        }

        let difficulty = env::var("DIFFICULTY");
        if let Err(e) = path {
            return Err(Box::new(LoadFileError(
                "Could not get DIFFICULTY".to_string(),
            )));
        }
        let difficulty = difficulty.unwrap().parse::<u64>();
        if let Err(e) = path {
            return Err(Box::new(LoadFileError(
                "Could not parse DIFFICULTY to u64".to_string(),
            )));
        }

        let reward = env::var("REWARD");
        if let Err(e) = path {
            return Err(Box::new(LoadFileError("Could not get REWARD".to_string())));
        }
        let reward = reward.unwrap().parse::<u64>();
        if let Err(e) = path {
            return Err(Box::new(LoadFileError(
                "Could not parse REWARD to u64".to_string(),
            )));
        }

        let genesis_prev_hash = env::var("GENESIS_PREV_HASH");
        if let Err(e) = path {
            return Err(Box::new(LoadFileError(
                "Could not get GENESIS_PREV_HASH".to_string(),
            )));
        }

        Ok(EnvConfig {
            difficulty: difficulty.unwrap(),
            reward: reward.unwrap(),
            genesis_prev_hash: genesis_prev_hash.unwrap(),
        })
    }
}

impl Config for EnvConfig {
    fn get_difficulty(&self) -> u64 {
        self.difficulty
    }
    fn get_reward(&self) -> u64 {
        return self.reward;
    }
    fn get_genesis_prev_hash(&self) -> String {
        return self.genesis_prev_hash.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, EnvConfig};

    #[test]
    fn test_config() {
        let config = EnvConfig::new(".env");
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(1, config.get_difficulty());
        assert_eq!(10, config.get_reward());
        assert_eq!(
            "0000000000000000000000000000000000000000000000000000000000000000",
            config.get_genesis_prev_hash()
        );
    }
}
