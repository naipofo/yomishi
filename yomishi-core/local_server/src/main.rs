mod connect;

use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use http::HeaderValue;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

use crate::connect::{RcpRequest, RpcMediator};

// TODO: make this *way* cleaner
#[tokio::main]
async fn main() {
    let mediator = Arc::new(RpcMediator::new());

    // [::1]:50051
    let addr = SocketAddr::from(([127, 0, 0, 1], 50051));

    let mediator = mediator.clone();
    let make_svc = make_service_fn(move |_| {
        let mediator = mediator.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let mediator = mediator.clone();
                async move {
                    let (h, body) = req.into_parts();

                    let mut path: Vec<_> = h
                        .uri
                        .path()
                        .to_string()
                        .split('/')
                        .skip(1)
                        .map(|e| e.to_string())
                        .collect();
                    let service = path.remove(0);
                    let method_name = path.remove(0);

                    let data = hyper::body::to_bytes(body).await.unwrap().to_vec();

                    let responce = mediator.rpc(RcpRequest {
                        service,
                        method_name,
                        data,
                    });
                    let mut r = Response::new(Body::from(responce));
                    r.headers_mut().append(
                        "Access-Control-Allow-Origin",
                        HeaderValue::from_str("*").unwrap(),
                    );
                    Ok::<_, Infallible>(r)
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
