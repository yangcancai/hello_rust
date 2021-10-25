use actor::hello::MyActorHandle;
use actor::actor::MyHandler;

#[tokio::test]
async fn hello(){
	let my = MyActorHandle::new();
	let a = my.get_unique_id().await;
	let b = my.get_unique_id().await;
	assert_eq!(a, 1);
	assert_eq!(b, 2);
	assert_eq!(my.get_unique_id().await, 3);
}
#[tokio::test]
async fn actor(){
	let my = MyHandler::new();
	assert_eq!(my.incr().await,Ok(1));
	assert_eq!(my.incr().await,Ok(2));
	assert_eq!(my.incr().await,Ok(3));
	my.close();
	assert_eq!(my.incr().await,Err(()));
}
