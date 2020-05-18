use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, middleware, web};
use actix_web::client::Client;
use rustls::{NoClientAuth, ResolvesServerCertUsingSNI, ServerConfig};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::sign::{RSASigningKey, SigningKey};

use crate::config::{Domain, Protocol, read_domain_config, read_proxy_config, get_forward_url};

fn add_certificate_to_resolver(domain: &Domain, resolver: &mut ResolvesServerCertUsingSNI) {
    let cert_file = &mut BufReader::new(File::open(domain.cert_chain.as_ref().unwrap()).unwrap());
    let key_file = &mut BufReader::new(File::open(&domain.cert_key.as_ref().unwrap()).unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    let signing_key = RSASigningKey::new(&keys.remove(0)).unwrap();
    let signing_key_boxed: Arc<Box<dyn SigningKey>> = Arc::new(Box::new(signing_key));

    resolver.add(&domain.name, rustls::sign::CertifiedKey::new(cert_chain, signing_key_boxed)).expect(&format!("Invalid certificate {}", domain.name));
}

async fn forward(
    req: HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse, Error> {

    let client = Client::new();

    let mut domain_name = String::from(req.connection_info().host());
    if domain_name.contains(":") {
        let mut parts = domain_name.split(":");
        domain_name = parts.next().unwrap().to_string();
    }

    let mut new_url = get_forward_url(&domain_name);
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    let forwarded_req = client
        .request_from(new_url.as_str(), req.head())
        .no_decompress();
    let forwarded_req = if let Some(addr) = req.head().peer_addr {
        forwarded_req.header("x-forwarded-for", format!("{}", addr.ip())).header("forwarded", format!("for={}", addr.ip()))
    } else {
        forwarded_req
    };

    let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());

    for (header_name, header_value) in
    res.headers().iter()
    {
        println!("Header name: {} value:{:?}", header_name.clone(), header_value.clone());
    }

    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in
    res.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.body(res.body().await?))
}

pub async fn start() -> std::io::Result<()> {
    let domain_configs = read_domain_config();
    let proxy_config = read_proxy_config();

    let mut resolver = ResolvesServerCertUsingSNI::new();

    for domain in domain_configs {
        if domain.cert_chain.is_some() {
            add_certificate_to_resolver(&domain, &mut resolver);
        }
    }

    let mut config = ServerConfig::new(NoClientAuth::new());
    config.cert_resolver = Arc::new(resolver);

    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(forward))
    });

    let address = format!("{}:{}", proxy_config.proxy_address, proxy_config.proxy_port);

    if proxy_config.proxy_protocol == Protocol::Https {
        server = server.bind_rustls(address, config)?;
    } else {
        server = server.bind(address)?
    }

    server.run().await
}