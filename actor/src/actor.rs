//-------------------------------------------------------------------
// @author yangcancai
// Copyright (c) 2021 by yangcancai(yangcancai0112@gmail.com), All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// @doc
//
// @end
// Created : 2021-10-25T03:50:57+00:00
//-------------------------------------------------------------------
use tokio::sync::{mpsc, oneshot};
struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    counter: i32,
}

enum ActorMessage {
    Close,
    Incr(oneshot::Sender<ActorReply<i32>>),
}
#[derive(Debug)]
enum ActorReply<T> {
    Ok(T),
}
pub struct MyHandler {
    sender: mpsc::Sender<ActorMessage>,
    join_handle: tokio::task::JoinHandle<()>,
}
impl MyActor {
    pub fn new(rec: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver: rec,
            counter: 0,
        }
    }
    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            if self.handle_message(msg) {
                break;
            }
        }
    }
    pub fn handle_message(&mut self, msg: ActorMessage) -> bool {
        match msg {
            ActorMessage::Close => true,
            ActorMessage::Incr(response) => {
                self.counter += 1;
                let _ = response.send(ActorReply::Ok(self.counter));
                false
            }
        }
    }
}

impl MyHandler {
    pub fn new() -> Self {
        let (s, r) = mpsc::channel(8);
        let mut actor = MyActor::new(r);
        let join_handler = tokio::spawn(async move { actor.run().await });
        MyHandler {
            sender: s,
            join_handle: join_handler,
        }
    }
    pub async fn incr(&self) -> Result<i32, ()> {
        let (s, r) = oneshot::channel();
        let _ = self.sender.send(ActorMessage::Incr(s)).await;
        match r.await {
            Ok(ActorReply::Ok(rs)) => Ok(rs),
            Err(_) => Err(()),
        }
    }
    pub fn close(&self) {
        let _ = self.sender.send(ActorMessage::Close);
        self.join_handle.abort();
    }
}
