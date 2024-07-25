use std::process;

mod curlsys;

pub struct Error(i32);

fn usage() {
    println!("usage: http3 curl_sys/curl URL REPEAT");
    process::exit(1);
}

fn main() {
    // Args processing
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        usage();
    }
    let kind = &args[1];
    let url = &args[2];
    let repeat = args[3].parse::<i32>().unwrap();

    for _ in 0..repeat {
        match kind.as_str() {
            "curl_sys" => {
                let ret = curlsys::perform_head(url);
                if let Err(Error(code)) = ret {
                    process::exit(code);
                }
            }
            _ => usage(),
        }
    }
}
