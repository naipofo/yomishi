use std::{
    collections::HashMap,
    rc::Rc,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread,
};

use yomishi_proto::yomishi::scan::ScanServer;

pub struct RcpRequest {
    pub service: String,
    pub method_name: String,
    pub data: Vec<u8>,
}

struct RcpResolver(HashMap<String, Rc<dyn yomishi_proto::ProtoService>>);

impl RcpResolver {
    fn new() -> Self {
        RcpResolver(HashMap::new())
    }
    fn add(&mut self, s: Rc<dyn yomishi_proto::ProtoService>) {
        self.0.insert(s.name().to_string(), s);
    }
    fn execute(
        &self,
        RcpRequest {
            service,
            method_name,
            data,
        }: RcpRequest,
    ) -> Vec<u8> {
        self.0.get(&service).unwrap().execute(&method_name, &data)
    }
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
    let backend = Rc::new(yomishi::backend::Backend::new().unwrap());
    let server = Rc::new(ScanServer(backend.clone()));
    let mut resolver = RcpResolver::new();
    resolver.add(server.clone());

    loop {
        let a = rx.recv().unwrap();
        let data = resolver.execute(a);
        tx.send(data).unwrap();
    }
}
