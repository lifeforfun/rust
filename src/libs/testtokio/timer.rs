use futures::lazy;
use log::info;
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Delay;

pub fn test() {
    tokio::run(lazy(|| {
        for i in 0..10 {
            let future_task = Delay::new(Instant::now() + Duration::from_secs(1))
                .and_then(move |t| {
                    println!("Hello {}, {:?}", i, t);
                    info!("task run: {}", i);
                    Ok(())
                })
                .map_err(|e| {
                    println!("error occur: {:?}", e);
                });
            tokio::spawn(future_task);
        }
        // closure must return Result to support `IntoFuture`
        Ok(())
    }));
}
