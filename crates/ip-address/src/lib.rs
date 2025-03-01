use image::*;
use qrcode::QrCode;
use qrcode::render::svg;
use std::error::Error;
use std::net::IpAddr;

pub type Err = Box<dyn Error>;

#[derive(Clone, PartialEq)]
pub struct IpDetails {
    pub ip: IpAddr,
    pub port: u16,
    pub ip_string: String,
    pub path: String,
    pub png: DynamicImage,
    pub svg_xml: String,
    pub is_https: bool,
    pub url: String,
}

impl std::fmt::Display for IpDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        buffer.push_str("ip_details:\n");
        buffer.push_str(&format!("ip: {}\n", self.ip));
        buffer.push_str(&format!("port: {}\n", self.port));
        buffer.push_str(&format!("ip_string: {}\n", self.ip_string));
        buffer.push_str(&format!("path: {}\n", self.path));
        buffer.push_str(&format!("is_https: {}\n", self.is_https));
        buffer.push_str(&format!("url: {}\n", self.url));
        write!(f, "{}", buffer)
    }
}

pub fn get_details(port: u16, path: &str, is_https: bool) -> Result<IpDetails, Err> {
    let local_ip = local_ip_address::local_ip().unwrap();
    let https = if is_https { "https" } else { "http" };
    let url = format!("{https}://{:?}:{}/{}", local_ip, port, path);

    let code = QrCode::new(&url).unwrap();

    // Render the bits into an image.
    let image = code.render::<Rgba<u8>>().build().into();

    let svg_xml = code.render::<svg::Color>().build();

    Ok(IpDetails {
        ip: local_ip,
        url,
        ip_string: format!("{}:{}", local_ip, port),
        port,
        path: path.to_string(),
        png: image,
        svg_xml,
        is_https,
    })
}
