use std::env;
use url::Url;

#[derive(Debug)]
pub enum ServicesName {
    WEB,
}

impl ServicesName {
    fn as_str(&self) -> &'static str {
        match self {
            ServicesName::WEB => "web",
        }
    }
}

fn get_service_port_by_name(service_name: &str) -> String {
    let port_key = format!("SERVICE_{}_PORT", service_name.to_uppercase());

    env::var(&port_key).expect(&format!(
        "Missing or invalid environment variable: {}",
        port_key
    ))
}

pub fn get_service_port(service: ServicesName) -> String {
    let service_name = service.as_str();
    get_service_port_by_name(service_name)
}

pub fn get_service_url(service: ServicesName) -> String {
    let service_name = service.as_str();
    let port = get_service_port_by_name(service_name);
    let url = format!("http://{}:{}/", service_name, port);

    Url::parse(&url).expect(&format!("Invalid URL format generated: {}", url));

    url
}
