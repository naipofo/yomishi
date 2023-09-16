#[allow(non_snake_case)]
pub mod yomishi {
    pub mod scan {
        tonic::include_proto!("yomishi.scan");
    }
    pub mod anki {
        tonic::include_proto!("yomishi.anki");
    }
}
