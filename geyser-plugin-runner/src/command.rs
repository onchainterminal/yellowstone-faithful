use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Semaphore;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct TcpCommandServer {
    semaphore: Arc<Semaphore>,
    rt: Arc<Runtime>,  // ✅ Hold our own runtime
}

impl TcpCommandServer {
    pub fn new(addr: &str) -> Arc<Self> {
        let semaphore = Arc::new(Semaphore::new(0));
        let rt = Arc::new(Runtime::new().unwrap());

        let semaphore_clone = semaphore.clone();
        let rt_clone = rt.clone();
        let addr = addr.to_string();

        // Run server setup in our runtime
        rt.block_on(async move {
            let listener = TcpListener::bind(&addr).await.unwrap();
            println!("Command server listening on {}", addr);

            tokio::spawn(async move {
                loop {
                    let (socket, _) = listener.accept().await.unwrap();
                    let reader = BufReader::new(socket);
                    let mut lines = reader.lines();

                    while let Ok(Some(line)) = lines.next_line().await {
                        println!("Command received: {}", line.trim());
                        if line.trim() == "go" {
                            semaphore_clone.add_permits(1);
                        }
                    }
                }
            });
        });

        Arc::new(TcpCommandServer { semaphore, rt })
    }

    /// Wait for a "go" command
    pub fn wait_for_go(&self) {
        let semaphore = self.semaphore.clone();
        // ✅ Use our own runtime instead of Handle::current()
        self.rt.block_on(async {
            semaphore.acquire().await.unwrap().forget();
        });
    }
}
