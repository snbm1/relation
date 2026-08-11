use anyhow::{Context, Result, anyhow, bail};
use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD},
};
use url::Url;

use crate::configurator::outbound::Outbound;
use crate::minireq::get_responce;

pub struct Subscription;

impl Subscription {
    pub fn load(input: &str) -> Result<Vec<Outbound>> {
        let url = Url::parse(input).context("Invalid subscription URL")?;

        match url.scheme() {
            "http" | "https" => {}
            scheme => {
                bail!("Unsupported subscription scheme '{scheme}'");
            }
        }

        let host = url
            .host_str()
            .context("Subscription URL does not contain a host")?;

        let port = url
            .port_or_known_default()
            .context("Cannot determine subscription port")?;

        let path = build_request_path(&url);

        let response = get_responce(host, port, &path)
            .map_err(|err| anyhow!("{err}"))
            .context("Cannot download subscription")?;

        Self::parse(&response)
    }

    pub fn parse(input: &str) -> Result<Vec<Outbound>> {
        let content = if looks_like_subscription_text(input) {
            input.to_owned()
        } else {
            let decoded =
                decode_base64_flexible(input).context("Cannot decode subscription as Base64")?;

            String::from_utf8(decoded).context("Subscription is not valid UTF-8")?
        };

        Self::parse_lines(&content)
    }

    fn parse_lines(input: &str) -> Result<Vec<Outbound>> {
        let mut outbounds = Vec::new();

        for (line_number, raw_line) in input.lines().enumerate() {
            let line = raw_line.trim();

            if line.is_empty() {
                continue;
            }
            if line.starts_with('#') {
                continue;
            }
            let outbound = Outbound::from_url(line).with_context(|| {
                format!(
                    "Cannot parse subscription entry at line {}",
                    line_number + 1
                )
            })?;

            outbounds.push(outbound);
        }

        if outbounds.is_empty() {
            bail!("Subscription does not contain any supported outbound URLs");
        }

        Ok(outbounds)
    }
}

fn build_request_path(url: &Url) -> String {
    let mut path = url.path().to_owned();

    if path.is_empty() {
        path.push('/');
    }

    if let Some(query) = url.query() {
        path.push('?');
        path.push_str(query);
    }

    path
}

fn looks_like_subscription_text(input: &str) -> bool {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .any(looks_like_outbound_url)
}

fn looks_like_outbound_url(input: &str) -> bool {
    let Ok(url) = Url::parse(input) else {
        return false;
    };

    matches!(
        url.scheme(),
        "vless"
            | "socks"
            | "socks4"
            | "socks4a"
            | "socks5"
            | "socks5h"
            | "http"
            | "https"
            | "trojan"
            | "ss"
            | "hysteria2"
            | "hy2"
            | "tuic"
            | "anytls"
    )
}

fn decode_base64_flexible(input: &str) -> Result<Vec<u8>> {
    let normalized: String = input.chars().filter(|ch| !ch.is_whitespace()).collect();

    if normalized.is_empty() {
        bail!("Empty Base64 subscription");
    }
    if let Ok(decoded) = STANDARD.decode(&normalized) {
        return Ok(decoded);
    }

    if let Ok(decoded) = STANDARD_NO_PAD.decode(&normalized) {
        return Ok(decoded);
    }

    if let Ok(decoded) = URL_SAFE.decode(&normalized) {
        return Ok(decoded);
    }

    if let Ok(decoded) = URL_SAFE_NO_PAD.decode(&normalized) {
        return Ok(decoded);
    }

    bail!("Unsupported or invalid Base64 encoding")
}
