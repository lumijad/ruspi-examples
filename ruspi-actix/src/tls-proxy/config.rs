use std::env;
use std::net::ToSocketAddrs;

use log::debug;
use url::Url;

use crate::config;
use std::collections::HashMap;

pub fn create_forward_url(domain: &config::Domain) -> Url {
    let protocol = match domain.protocol {
        Protocol::Http => "http",
        Protocol::Https => "https"
    };

    let address = format!("{}:{}", domain.address, domain.port);

    Url::parse(&format!(
        "{}://{}", protocol, address
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
    )).unwrap()
}


#[derive(Debug)]
pub struct Domain {
    pub name: String,
    pub protocol: Protocol,
    pub address: String,
    pub port: u16,
    pub timeout_in_seconds: u16,
    pub cert_chain: Option<String>,
    pub cert_key: Option<String>,
}


#[derive(Debug, PartialEq)]
pub enum Protocol {
    Http,
    Https,
}

impl std::str::FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http" => Ok(Protocol::Http),
            "https" => Ok(Protocol::Https),
            _ => Err(format!("'{}' is not a valid value for Protocol", s)),
        }
    }
}

impl Domain {
    pub fn new(name: &str) -> Self {
        Domain {
            name: env::var(format!("DOMAIN.{}.NAME", name)).unwrap(),
            protocol: env::var(format!("DOMAIN.{}.PROTOCOL", name)).unwrap().parse().unwrap(),
            address: env::var(format!("DOMAIN.{}.ADDRESS", name)).unwrap(),
            port: env::var(format!("DOMAIN.{}.PORT", name)).unwrap().parse::<u16>().unwrap(),
            timeout_in_seconds: env::var(format!("DOMAIN.{}.TIMEOUT_IN_SECONDS", name)).unwrap().parse::<u16>().unwrap(),
            cert_chain: env::var(format!("DOMAIN.{}.CERT_CHAIN", name)).ok(),
            cert_key: env::var(format!("DOMAIN.{}.CERT_KEY", name)).ok(),
        }
    }
}

pub fn read_domain_config() -> Vec<config::Domain> {
    let domains = env::var("DOMAINS").unwrap();
    let names = domains.split(",");
    let mut domains = Vec::<Domain>::new();

    for name in names {
        debug!("Reading config for domain {}", name);
        domains.push(Domain::new(name));
    }

    domains
}




lazy_static! {
    pub static ref FORWARD_URLS: HashMap<String, Url> = {

        let mut map: HashMap::<String, Url> = HashMap::new();

        for domain in read_domain_config() {
            map.insert(domain.name.clone(), create_forward_url(&domain));
        }

        map
    };
}

pub fn get_forward_url(domain: &String) -> Url {
    FORWARD_URLS.get(domain).unwrap().clone()
}


#[derive(Debug)]
pub struct Proxy {
    pub proxy_protocol: Protocol,
    pub proxy_address: String,
    pub proxy_port: u16,

    pub management_protocol: Protocol,
    pub management_address: String,
    pub management_port: u16,
    pub management_cert_chain: Option<String>,
    pub management_cert_key: Option<String>,
}


impl Proxy {
    pub fn new() -> Self {
        Proxy {
            proxy_protocol: env::var("PROXY_PROTOCOL").unwrap().parse().unwrap(),
            proxy_address: env::var("PROXY_ADDRESS").unwrap(),
            proxy_port: env::var("PROXY_PORT").unwrap().parse::<u16>().unwrap(),

            management_protocol: env::var("MANAGEMENT_PROTOCOL").unwrap().parse().unwrap(),
            management_address: env::var("MANAGEMENT_ADDRESS").unwrap(),
            management_port: env::var("MANAGEMENT_PORT").unwrap().parse::<u16>().unwrap(),
            management_cert_chain: env::var("MANAGEMENT_CERT_CHAIN").ok(),
            management_cert_key: env::var("MANAGEMENT_CERT_KEY").ok(),
        }
    }
}

pub fn read_proxy_config() -> config::Proxy {
    Proxy::new()
}

