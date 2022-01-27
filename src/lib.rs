use futures::TryFutureExt;
use idna::domain_to_ascii;
use regex::Regex;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::runtime::Builder;

pub mod error;
pub type Result<T> = std::result::Result<T, error::Error>;

const PUBLIC_SUFFIX_LIST_URLS: &[&str] = &[
    "https://publicsuffix.org/list/public_suffix_list.dat",
    "https://raw.githubusercontent.com/publicsuffix/list/master/public_suffix_list.dat",
];

const PUBLIC_SUFFIX_RE: &str = r"^(?P<suffix>[.*!]*\w[\S]*)";

pub fn download(include_private_domains: bool) -> Result<Vec<String>> {
    //! Downloads the public tld suffixes
    //!
    //! ## Example Usage
    //! ```rust
    //! use tld_download::download;
    //!
    //! let suffixes = tld_download::download(false);
    //! assert!(suffixes.is_ok())
    //! ```
    let rt = Builder::new_current_thread().enable_all().build()?;
    let reg = Regex::new(PUBLIC_SUFFIX_RE)?;
    let x = Arc::new(Mutex::new(vec![]));
    let mut res = HashSet::new();
    for u in PUBLIC_SUFFIX_LIST_URLS {
        let respfut = reqwest::get(*u);
        let contentfut = respfut.and_then(|resp| resp.bytes());
        let content = rt.block_on(contentfut)?;
        x.lock()?.push(content);
    }
    let x = x
        .lock()?
        .to_vec()
        .into_iter()
        .map(|v| String::from_utf8_lossy(&*v).to_string())
        .collect::<Vec<String>>();

    for buf in x {
        let lines = if !include_private_domains {
            buf.split("// ===BEGIN PRIVATE DOMAINS===")
                .next()
                .unwrap_or("")
        } else {
            &buf[..]
        }
        .lines();

        for line in lines {
            if !line.starts_with("//") {
                let cap = reg.captures(line.trim()).and_then(|cap| cap.name("suffix"));
                if let Some(m) = cap {
                    let r = match domain_to_ascii(m.as_str()) {
                        Ok(caps) => caps,
                        Err(_) => m.as_str().to_string(),
                    };
                    res.insert(r);
                }
            }
        }
    }

    Ok(Vec::from_iter(res))
}
