use crate::{exit, util};
use globset::Glob;
use hyper::header::{HeaderName, HeaderValue};
use hyper::{Method, Uri};
use regex::Regex;
use std::net::{IpAddr, SocketAddr};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use util::{try_parse_duration, try_parse_size, try_to_socket_addr};

// Get the extension of the path
pub trait PathExtension {
    fn get_extension(&self) -> Option<&str>;
}

impl PathExtension for PathBuf {
    fn get_extension(&self) -> Option<&str> {
        self.extension()
            .map(|ext| ext.to_str())
            .unwrap_or_else(|| Some(""))
    }
}

// Convert path to absolute path
pub trait AbsolutePath {
    fn absolute_path<P: AsRef<Path>>(&self, root: P) -> PathBuf;
}

impl AbsolutePath for String {
    fn absolute_path<P: AsRef<Path>>(&self, root: P) -> PathBuf {
        let path = PathBuf::from(self);
        if path.is_absolute() {
            path
        } else {
            root.as_ref().join(self)
        }
    }
}

// Force conversion of string to specified type
pub trait Force {
    fn to_duration(&self) -> Duration;
    fn to_size(&self) -> usize;
    fn to_glob(&self) -> Glob;
    fn to_header_name(&self) -> HeaderName;
    fn to_header_value(&self) -> HeaderValue;
    fn to_method(&self) -> Method;
    fn to_regex(&self) -> Regex;
    fn to_socket_addr(&self) -> SocketAddr;
    fn to_ip_addr(&self) -> IpAddr;
    fn to_strftime(&self);
    fn to_uri(&self) -> Uri;
}

impl Force for &str {
    fn to_duration(&self) -> Duration {
        try_parse_duration(self).unwrap_or_else(|err| {
            exit!("Cannot parse `{}` to duration: {}", self, err.description())
        })
    }

    fn to_size(&self) -> usize {
        try_parse_size(self)
            .unwrap_or_else(|err| exit!("Cannot parse `{}` to size: {}", self, err.description()))
    }

    fn to_glob(&self) -> Glob {
        Glob::new(self)
            .unwrap_or_else(|err| exit!("Cannot parse `{}` to glob matcher\n{}", self, err))
    }

    fn to_header_name(&self) -> HeaderName {
        HeaderName::from_str(self)
            .unwrap_or_else(|err| exit!("Cannot parse `{}` to http header name\n{}", self, err))
    }

    fn to_header_value(&self) -> HeaderValue {
        HeaderValue::from_str(self)
            .unwrap_or_else(|err| exit!("Cannot parse `{}` to http header value\n{}", self, err))
    }

    fn to_method(&self) -> Method {
        Method::from_str(self)
            .unwrap_or_else(|err| exit!("Cannot parse `{}` to http method\n{}", self, err))
    }

    fn to_regex(&self) -> Regex {
        Regex::new(self)
            .unwrap_or_else(|err| exit!("Cannot parse `{}` to regular expression\n{}", self, err))
    }

    fn to_socket_addr(&self) -> SocketAddr {
        try_to_socket_addr(self).unwrap_or_else(|_| exit!("Cannot parse `{}` to SocketAddr", self))
    }

    fn to_ip_addr(&self) -> IpAddr {
        self.parse::<IpAddr>()
            .unwrap_or_else(|_| exit!("Cannot parse `{}` to IP addr", self))
    }

    fn to_strftime(&self) {
        time::now()
            .strftime(self)
            .unwrap_or_else(|err| exit!("Cannot parse `{}` to time format\n{}", self, err));
    }

    fn to_uri(&self) -> Uri {
        self.parse::<Uri>()
            .unwrap_or_else(|err| exit!("Cannot parse `{}` to http uri\n{}", self, err))
    }
}
