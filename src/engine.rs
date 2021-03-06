#[allow(unused_imports)]
use std::time::{Duration, Instant, SystemTime};
// #[allow(unused_imports)]
use std::thread::{sleep, yield_now};

// use settimeout::set_timeout;
#[allow(unused_imports)]
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};

// use crate::object;


pub async fn start(){
    loop {
        yield_now();
    }
}




#[allow(dead_code)]
pub async fn test() {
    // let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);

    // let now = Local::now().timestamp_millis();
    // println!("{}", now);
    let mut i = 0;
    let mut now;
    loop {
        // let now2 = Instant::now();
        now = Instant::now();
        // let d = now2.elapsed().as_millis() as u64;
        // sleep(Duration::from_nanos(100));
        // set_timeout(Duration::from_millis(16)).await;
        // sleep(Duration::new(0,10000));
        // sleep_ms(15);
        sleep(Duration::from_nanos(100000));
        // yield_now();
        println!("{}", now.elapsed().as_millis());

        i += 1;
        if i >= 100 {
            break;
        }
    }

    // we sleep for 2 seconds

    // it prints '2'
    //    println!("{}", now.elapsed().as_secs());

    //    sleep(Duration::new(2, 0));

    //    println!("{}", now.elapsed().as_secs());
}


#[cfg(test)]
mod tests {

    use crate::engine;

    #[test]
    fn engine_test() {
        engine::start();
    }
}