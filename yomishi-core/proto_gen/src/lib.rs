use std::fmt::Write;

use prost_build::{Method, Service, ServiceGenerator};

pub struct ServiceGen;

// TODO: error handling
impl ServiceGenerator for ServiceGen {
    fn generate(&mut self, service: Service, buf: &mut String) {
        let Service {
            name,
            methods,
            proto_name,
            ..
        } = service;
        // imports
        buf.push_str("use prost::Message;");

        // trait
        writeln!(buf, "pub trait {name} {{").unwrap();
        for Method {
            name,
            input_type,
            output_type,
            ..
        } in &methods
        {
            writeln!(
                buf,
                "fn {name}(&self, data: {input_type}) -> {output_type};\n"
            )
            .unwrap();
        }
        buf.push('}');

        // TODO: do I really need it?
        writeln!(
            buf,
            "pub struct {name}Server<T: {name}>(pub std::rc::Rc<T>);"
        )
        .unwrap();

        writeln!(
            buf,
            "impl<T: {name}> crate::ProtoService for {name}Server<T>{{"
        )
        .unwrap();

        buf.push_str("fn execute(&self, method_name: &str, data: &[u8]) -> Vec<u8>{");
        buf.push_str("match method_name{");
        for Method {
            name,
            proto_name,
            input_type,
            ..
        } in &methods
        {
            writeln!(
                buf,
                "\"{proto_name}\" => self.0.{name}({input_type}::decode(data).unwrap()).encode_to_vec(), "
            )
            .unwrap();
        }
        buf.push_str(" _ => panic!()");
        buf.push_str("}}");
        writeln!(buf, "fn name(&self) -> &'static str{{\"{proto_name}\"}}").unwrap();
        buf.push('}');
    }
}
