use std::{
    rc::Rc,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread,
};

use tokio::runtime::Runtime;
use yomishi_proto::{
    yomishi::{
        anki::{Anki, AnkiServer},
        config::{Config, ConfigServer},
        scan::{Scan, ScanServer},
    },
    ProtoService,
};

pub struct RcpRequest {
    pub service: String,
    pub method_name: String,
    pub data: Vec<u8>,
}

pub struct RpcMediator(Mutex<RpcMediatorInner>);

struct RpcMediatorInner {
    tx: Sender<RcpRequest>,
    rx: Receiver<Vec<u8>>,
}

impl RpcMediator {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let (resolver_tx, resolver_rx) = mpsc::channel();

        thread::spawn(move || {
            resolver_thread(rx, resolver_tx);
        });

        RpcMediator(Mutex::new(RpcMediatorInner {
            tx,
            rx: resolver_rx,
        }))
    }
    pub fn rpc(&self, request: RcpRequest) -> Vec<u8> {
        let inner = self.0.lock().unwrap();
        inner.tx.send(request).unwrap();
        inner.rx.recv().unwrap()
    }
}
fn resolver_thread(rx: mpsc::Receiver<RcpRequest>, tx: mpsc::Sender<Vec<u8>>) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    // TODO: move this somewhere else
    let backend = Rc::new(runtime.block_on(yomishi::backend::Backend::new()).unwrap());
    // let mut resolver = RcpResolver::new();

    loop {
        let a = rx.recv().unwrap();
        let data = execute(
            backend.clone(),
            &a.service,
            &a.method_name,
            &a.data,
            &runtime,
        );

        tx.send(data).unwrap();
    }
}

fn execute<T>(bc: Rc<T>, service: &str, metod: &str, data: &[u8], runtime: &Runtime) -> Vec<u8>
where
    T: Scan + Anki + Config,
{
    let scan: ScanServer<T> = ScanServer(bc.clone());
    let anki = AnkiServer(bc.clone());
    let config = ConfigServer(bc.clone());

    runtime.block_on(async {
        match service {
            "Scan" => scan.execute(metod, data).await,
            "Anki" => anki.execute(metod, data).await,
            "Config" => config.execute(metod, data).await,
            _ => panic!(),
        }
    })
}
