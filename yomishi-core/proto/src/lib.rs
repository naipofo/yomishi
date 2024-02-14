pub mod yomishi {
    pub mod scan {
        include!(concat!(env!("OUT_DIR"), "/yomishi.scan.rs"));
    }
    pub mod config {
        include!(concat!(env!("OUT_DIR"), "/yomishi.config.rs"));
    }
    pub mod anki {
        include!(concat!(env!("OUT_DIR"), "/yomishi.anki.rs"));
    }
}

#[allow(async_fn_in_trait)]
pub trait ProtoService {
    // TODO: use index instead of str
    async fn execute(&self, method_name: &str, data: &[u8]) -> Vec<u8>;
    fn name(&self) -> &'static str;
}
