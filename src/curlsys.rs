use crate::Error;
use curl_sys::{CURLcode, CURLE_OK};
use std::ffi::{c_long, CString};

pub fn perform_head(url: &str) -> Result<(), Error> {
    unsafe {
        curl_sys::curl_global_init(curl_sys::CURL_GLOBAL_ALL);

        let handle = curl_sys::curl_easy_init();

        let url = CString::new(url).unwrap();

        conv(curl_sys::curl_easy_setopt(
            handle,
            curl_sys::CURLOPT_URL,
            url.as_ptr(),
        ))?;
        conv(curl_sys::curl_easy_setopt(
            handle,
            curl_sys::CURLOPT_NOBODY,
            1 as c_long,
        ))?;
        conv(curl_sys::curl_easy_setopt(
            handle,
            curl_sys::CURLOPT_HTTP_VERSION,
            curl_sys::CURL_HTTP_VERSION_3 as c_long,
        ))?;
        conv(curl_sys::curl_easy_setopt(
            handle,
            curl_sys::CURLOPT_VERBOSE,
            1 as c_long,
        ))?;
        conv(curl_sys::curl_easy_setopt(
            handle,
            curl_sys::CURLOPT_TIMEOUT_MS,
            20 * 1000 as c_long,
        ))?;
        conv(curl_sys::curl_easy_perform(handle))?;

        Ok(())
    }
}

fn conv(code: CURLcode) -> Result<(), Error> {
    if code == CURLE_OK {
        Ok(())
    } else {
        Err(Error(code as i32))
    }
}
