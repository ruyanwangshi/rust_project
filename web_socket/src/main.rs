use std::collections::HashMap;
use std::hash::Hash;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// 添加静态全局key
static NEXT_USERID: AtomicUsize = AtomicUsize::new(1);

use futures::lock::Mutex;
use log;

#[tokio::main]
// #[tokio:main] 已经弃用
async fn main() {
    #[warn(unused_doc_comments)]
    /**
     * # 用Map来存储不同的用户
     * 1. K: id(usize)
     * 2. V: 发送给前端不同用户的渠道
     * # 通过Filter system warp 实现了
     * 1. Ws以及on_upgrade方法
     * 2. Websockets以及split方法
     * # 通过tokio的channel获取所有的前端发来的信息(WebSocket)转发给Map中V
     */

    log::debug!("RUST_APP_LOG");
    let map: Arc<Mutex<HashMap<_, _>>> = Arc::new(Mutex::new(HashMap::new()));

    let file = warp::fs::dir("static");
    warp::serve(file)
        .run(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            8080,
        ))
        .await;

    println!("Hello, world!");
}
