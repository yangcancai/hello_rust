use actor::actor::MyHandler;
use actor::hello::MyActorHandle;

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
    assert_eq!(my.incr().await, Ok(1));
    assert_eq!(my.incr().await, Ok(2));
    assert_eq!(my.incr().await, Ok(3));
    my.close();
    assert_eq!(my.incr().await, Err(()));
}
#[test]
fn block() {
    use tokio::runtime::Runtime;
    // Create the runtime
    let rt = Runtime::new().unwrap();
    // Execute the future, blocking the current thread until completion
    let _a = rt.enter();
    let my = MyHandler::new();
    let res = rt.block_on(async {
    my.incr().await
    });
    assert_eq!(res, Ok(1));
   let res = rt.block_on(async {
    my.incr().await
    });
    assert_eq!(res, Ok(2));

}
