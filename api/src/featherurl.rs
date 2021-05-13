use regex::Regex;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^((?P<scheme>[^:/?#]+):(//))?(//)?(((?P<login>[^:]+)(?::(?P<password>[^@]+)?)?@)?(?P<host>[^@/?#:]*)(?::(?P<port>\d+)?)?)?(?P<path>[^?#]*)(\?(?P<query>[^#]*))?(#(?P<fragment>.*))?").unwrap();
    static ref PATH_REGEX: Regex = Regex::new("/(?P<path>[^/]*)").unwrap();
}

const URI_CODES: [(char, &str); 24] = [
    (' ', "%20"),
    ('\"', "%22"),
    ('#', "%23"),
    ('$', "%24"),
    ('%', "%25"),
    ('&', "%26"),
    ('+', "%2B"),
    (',', "%2C"),
    ('/', "%2F"),
    (':', "%3A"),
    (';', "%3B"),
    ('<', "%3C"),
    ('=', "%3D"),
    ('>', "%3E"),
    ('?', "%3F"),
    ('@', "%40"),
    ('[', "%5B"),
    ('\\', "%5C"),
    (']', "%5D"),
    ('^', "%5E"),
    ('`', "%60"),
    ('{', "%7B"),
    ('|', "%7C"),
    ('}', "%7D"),
];

#[derive(Clone)]
pub struct FeatherUrl {
    protocol: String,
    hostname: String,
    port: i32,
    path: Vec<String>,
    file: String,
    query: HashMap<String, String>,
    hash: String,
    pathlock: usize,
}

impl From<&str> for FeatherUrl {
    fn from(url: &str) -> Self {
        let caps = REGEX.captures(url).unwrap();
        let protocol = match caps.name("scheme") {
            Some(a) => a.as_str(),
            _ => "http",
        };
        let hostname = &caps["host"];

        let port_raw = caps.name("port");
        let path_raw = caps.name("path");
        let query_raw = caps.name("query");
        let hash_raw = caps.name("fragment");

        let port: i32 = match port_raw {
            Some(mat) => mat.as_str().parse::<i32>().unwrap(),
            _ => -1,
        };

        let mut path: Vec<String> = Vec::new();
        let mut file: String = String::from("");
        if let Some(pat) = path_raw {
            let matches: regex::CaptureMatches = PATH_REGEX.captures_iter(pat.as_str());
            let mut paths: Vec<String> =
                matches.map(|pr| pr["path"].to_string()).collect::<Vec<_>>();
            if let Some(fi) = paths.pop() {
                file = fi;
            }
            path.extend(paths);
        }

        let query: HashMap<String, String> = match query_raw {
            Some(mat) => {
                let mut res: HashMap<String, String> = HashMap::new();
                let queries: Vec<&str> = mat.as_str().split('&').collect();
                for queri in queries {
                    let querii: Vec<&str> = queri.split('=').collect();
                    let val: &str = {
                        if querii.len() > 1 {
                            querii[1]
                        } else {
                            ""
                        }
                    };
                    res.insert(String::from(querii[0]), String::from(val));
                }
                res
            }
            _ => HashMap::new(),
        };

        let hash: String = match hash_raw {
            Some(mat) => String::from(mat.as_str()),
            _ => String::from(""),
        };

        Self {
            protocol: String::from(protocol),
            hostname: String::from(hostname),
            port,
            path,
            file,
            query,
            hash,
            pathlock: 0,
        }
    }
}

impl ToString for FeatherUrl {
    fn to_string(&self) -> String {
        let mut afterpart: String = String::from("");
        if self.port > -1 {
            afterpart.push(':');
            afterpart.push_str(&self.port.to_string());
        }
        afterpart.push('/');
        afterpart.push_str(&self.path.join("/"));
        if !self.path.is_empty() {
            afterpart.push('/');
        }
        afterpart.push_str(&self.file);
        if !self.query.is_empty() {
            let mut is_first = true;
            afterpart.push('?');
            for (key, value) in &self.query {
                if is_first {
                    is_first = false;
                } else {
                    afterpart.push('&');
                }
                afterpart.push_str(&key);
                afterpart.push('=');
                afterpart.push_str(&value);
            }
        }
        if !self.hash.is_empty() {
            afterpart.push('#');
            afterpart.push_str(&self.hash);
        }
        format!("{}://{}{}", self.protocol, self.hostname, afterpart)
    }
}

impl FeatherUrl {
    pub fn path(&self) -> String {
        format!("/{}", self.path.join("/"))
    }
    pub fn full_path(&self) -> String {
        let mut res: String = self.path();
        if !res.ends_with('/') {
            res.push('/');
        }
        res.push_str(&self.file);
        res
    }
    pub fn hostname_and_port(&self) -> String {
        let mut port: i32 = self.port;
        if port < 0 {
            port = 80;
        }
        format!("{}:{}", self.hostname, port.to_string())
    }
    pub fn to_string_basic(&self) -> String {
        let mut afterpart: String = String::from("");
        if self.port > -1 {
            afterpart.push(':');
            afterpart.push_str(&self.port.to_string());
        }
        afterpart.push('/');
        afterpart.push_str(&self.path.join("/"));
        if !self.path.is_empty() {
            afterpart.push('/');
        }
        afterpart.push_str(&self.file);
        format!("{}://{}{}", self.protocol, self.hostname, afterpart)
    }
    pub fn get_query(&self, key: &str) -> String {
        decode_uri_component(self.query.get(key).unwrap_or(&String::from("")))
    }
    pub fn get_raw_query(&self, key: &str) -> &String {
        &self.query.get(key).unwrap()
    }
    pub fn has_query(&self, key: &str) -> bool {
        self.query.contains_key(key)
    }
    pub fn raw_path(&self) -> &Vec<String> {
        &self.path
    }
    pub fn file(&self) -> &String {
        &self.file
    }
    pub fn hostname(&self) -> &String {
        &self.hostname
    }
    pub fn protocol(&self) -> &String {
        &self.protocol
    }
    pub fn port(&self) -> &i32 {
        &self.port
    }
    pub fn raw_query(&self) -> &HashMap<String, String> {
        &self.query
    }
    pub fn path_lock(&self) -> &usize {
        &self.pathlock
    }
    pub fn hash(&self) -> &String {
        &self.hash
    }

    //Mutable
    pub fn join(&mut self, path_a: &str) {
        let mut chars = path_a.chars();
        if path_a.starts_with("/") {
            chars.next();
        } else if path_a.starts_with("./") {
            chars.next();
            chars.next();
        }
        let path = chars.as_str();
        let mut path_parts: Vec<&str> = path.split('/').collect();
        self.file = String::from(path_parts.pop().unwrap_or(""));
        for part in path_parts {
            if part == ".." {
                if self.path.len() > self.pathlock {
                    self.path.pop();
                }
            } else {
                self.path.push(String::from(part));
            }
        }
    }
    pub fn set_path_lock(&mut self, path_lock: usize) {
        self.pathlock = path_lock;
    }
    pub fn add_query(&mut self, key: &str, value: &str) {
        self.query
            .insert(String::from(key), encode_uri_component(value));
    }
    pub fn remove_query(&mut self, key: &str) {
        self.query.remove(key);
    }
    pub fn set_query(&mut self, key: &str, value: &str) {
        *self.query.get_mut(key).unwrap() = encode_uri_component(value);
    }
    pub fn set_hash(&mut self, hash: &str) {
        self.hash = String::from(hash);
    }
    pub fn set_port(&mut self, port: i32) {
        self.port = port;
    }
    pub fn set_hostname(&mut self, hostname: &str) {
        self.hostname = String::from(hostname);
    }
    pub fn set_protocol(&mut self, protocol: &str) {
        self.protocol = String::from(protocol);
    }
}

pub fn encode_uri_component(component: &str) -> String {
    let mut res = String::from(component);
    for (uri_char, uri_replace) in URI_CODES.clone().iter() {
        res = res.replace(&uri_char.to_string(), uri_replace);
    }
    res
}

pub fn decode_uri_component(component: &str) -> String {
    let mut res = String::from(component);
    for (uri_replace, uri_char) in URI_CODES.clone().iter() {
        res = res.replace(uri_char, &uri_replace.to_string());
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::featherurl::FeatherUrl;

    #[test]
    fn url_from_and_to_string() {
        assert_eq!(
            "http://localhost:3000/",
            FeatherUrl::from("http://localhost:3000/").to_string()
        );
    }
    #[test]
    fn url_with_path() {
        assert_eq!(
            "http://localhost:3000/foo",
            FeatherUrl::from("http://localhost:3000/foo").to_string()
        );
    }
    #[test]
    fn url_with_different_path() {
        assert_ne!(
            "http://localhost:3000/foo",
            FeatherUrl::from("http://localhost:3000/bar").to_string()
        );
    }
    #[test]
    fn url_hash_and_query_order() {
        assert_eq!(
            "http://localhost:3000/foo?take=give#lol",
            FeatherUrl::from("http://localhost:3000/foo?take=give#lol").to_string()
        )
    }
    #[test]
    fn paths() {
        let url = "http://localhost:3000/foo/bar/";
        assert_eq!(FeatherUrl::from(url).path(), "/foo/bar");
    }
    #[test]
    fn hashes() {
        assert_eq!(
            FeatherUrl::from("http://localhost:3000/bar#foo").hash,
            "foo"
        );
    }
    #[test]
    fn files() {
        assert_eq!(
            FeatherUrl::from("http://localhost:3000/foo/bar/pur").file,
            "pur"
        );
    }
    #[test]
    fn join_path() {
        let mut url = FeatherUrl::from("http://localhost:3000/foo/bar");
        url.join("../bar");
        assert_eq!(url.full_path(), "/bar");
    }
    #[test]
    fn path_lock() {
        let mut url = FeatherUrl::from("http://localhost:3000/docs/feather/introduction");
        url.set_path_lock(1);
        url.join("../../bar");
        assert_eq!(url.full_path(), "/docs/bar");
    }
    #[test]
    fn hardcore_query() {
        let mut url = FeatherUrl::from("http://localhost:3000/test?foo=bar");
        assert!(url.has_query("foo"));
        assert_eq!(url.get_query("foo"), "bar");
        url.set_query("foo", "foo");
        assert_eq!(url.get_query("foo"), "foo");
        url.remove_query("foo");
        assert!(!url.has_query("foo"));
        url.add_query("some", "thing");
        assert!(url.has_query("some"));
        assert_eq!(url.get_query("some"), "thing");
        url.set_query("some", "thing/foo");
        assert_eq!(url.get_query("some"), "thing/foo"); /*Gamble: Sometimes fails, sometimes works*/
        assert_eq!(url.get_raw_query("some"), "thing%2Ffoo");
    }
}
