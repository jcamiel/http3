use crate::Error;
use curl::easy::{Easy, HttpVersion};
use std::time::Duration;

impl From<curl::Error> for Error {
    fn from(err: curl::Error) -> Self {
        Error(err.code() as i32)
    }
}

pub fn perform_head(url: &str) -> Result<(), Error> {
    let mut handle = Easy::new();
    handle.url(url)?;
    handle.nobody(true)?;
    handle.http_version(HttpVersion::V3)?;
    handle.verbose(true)?;
    handle.timeout(Duration::from_secs(20))?;

    handle.ssl_ctx_function(|_| Ok(()))?;

    let transfer = handle.transfer();
    transfer.perform()?;

    Ok(())
}
