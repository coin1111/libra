// 0L
//

use std::env;
pub struct Config {
    pub swarm_path: String,
    pub swarm_persona: String,
}

pub fn read_config() -> Result<Config,String>{
    let swarm_path = env::var("SWARM_PATH")
        .map_err(|err|err.to_string())?;
    let swarm_persona = env::var("SWARM_PERSONA")
        .map_err(|err|err.to_string())?;
    Ok(Config{
        swarm_path,
        swarm_persona,
    })
}
