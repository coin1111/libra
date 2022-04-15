// 0L
//

use std::env;
use std::str::FromStr;
use std::path::PathBuf;
use diem_types::waypoint::Waypoint;
use reqwest::Url;
pub struct TxConfig {
    pub url_opt: Option<Url>,
    pub waypoint: Option<Waypoint>,
    pub swarm_path: Option<PathBuf>,
    pub swarm_persona: Option<String>,
}

pub fn read_config() -> Result<TxConfig,String>{
    let url_opt = match env::var("SWARM_URL")
        .map_err(|err|err.to_string())
        .and_then(|v|Url::from_str(v.as_str())
            .map_err(|err|err.to_string())){
        Ok(url) => Some(url),
        Err(err) => {
            println!("Can't find SWARM_URL, {:?}",err);
            None
        }
    };
    let waypoint = match env::var("SWARM_WAYPOINT")
        .map_err(|err|err.to_string())
        .and_then(|s|Waypoint::from_str(s.as_str())
            .map_err(|err|err.to_string())) {
        Ok(w) => Some(w),
        Err(err) => {
            println!("Can't find SWARM_WAYPOINT, {:?}",err);
            None
        }
    };
    let swarm_path = match env::var("SWARM_PATH")
        .map_err(|err|err.to_string())
        .and_then(|v|PathBuf::from_str(v.as_str())
            .map_err(|err|err.to_string())){
        Ok(p) => Some(p),
        Err(err) => {
            println!("Can't find SWARM_PATH, {:?}",err);
            None
        }
    };
    let swarm_persona = match env::var("SWARM_PERSONA")  {
        Ok(s) => Some(s),
        Err(err) => {
            println!("Can't find SWARM_PERSONA, {:?}",err);
            None
        }
    };
    Ok(TxConfig{
        url_opt,
        waypoint,
        swarm_path,
        swarm_persona,
    })
}
