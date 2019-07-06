use tokio::prelude::*;
use tokio::timer::Delay;
use log::{info};
use std::time::{Duration, Instant};

pub fn test()
{
    let when = Instant::now() + Duration::from_millis(100);
    let task = Delay::new(when)
        .and_then(|_|{
            println!("Hello World!");
            info!("this is a log file hahaha");
            Ok(())
        })
        .map_err(|e| println!("delay errored; err={:?}", e))
        ;
    tokio::run(task)
}