pub mod yomishi {
    pub mod scan {
        include!(concat!(env!("OUT_DIR"), "/yomishi.scan.rs"));
    }
    pub mod config {
        include!(concat!(env!("OUT_DIR"), "/yomishi.config.rs"));
    }
}

pub trait ProtoService {
    // TODO: use index instead of str
    fn execute(&self, method_name: &str, data: &[u8]) -> Vec<u8>;
    fn name(&self) -> &'static str;
}
