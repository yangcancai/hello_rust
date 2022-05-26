use actor::actor::MyHandler;
use actor::hello::MyActorHandle;
use core::time;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::test]
async fn hello() {
    let my = MyActorHandle::new();
    let a = my.get_unique_id().await;
    let b = my.get_unique_id().await;
    assert_eq!(a, 1);
    assert_eq!(b, 2);
    assert_eq!(my.get_unique_id().await, 3);
}
#[tokio::test]
async fn actor() {
    let my = MyHandler::new();
    for i in 1..100 {
        assert_eq!(my.incr().await, Ok(i));
    }

    my.close();
    assert_eq!(my.incr().await, Err(()));
}
#[test]
fn multiple_call() {
    use tokio::runtime::Runtime;
    // Create the runtime
    let rt = Runtime::new().unwrap();
    // Execute the future, blocking the current thread until completion
    let _a = rt.enter();
    let my = MyHandler::new();
    let res = rt.block_on(async { my.incr().await });
    assert_eq!(res, Ok(1));
    let (s, r) = std::sync::mpsc::channel();
    let join = std::thread::spawn(move || {
        let _res = rt.block_on(async { my.incr().await });
        let res = rt.block_on(async { my.incr().await });
        s.send(res);
    });
    let rt = Runtime::new().unwrap();
    let a = r.recv();
    assert_eq!(a.unwrap(), Ok(3));
    join.join();
}

#[test]
fn block() {
    use tokio::runtime::Runtime;
    // Create the runtime
    let rt = Runtime::new().unwrap();
    // Execute the future, blocking the current thread until completion
    let _a = rt.enter();
    let my = MyHandler::new();
    let res = rt.block_on(async { my.incr().await });
    assert_eq!(res, Ok(1));
    let res = rt.block_on(async { my.incr().await });
    assert_eq!(res, Ok(2));
}
fn timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}
