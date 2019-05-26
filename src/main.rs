#![feature(async_await)]

mod modes;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::net::Ipv4Addr;
use std::path::PathBuf;

use hyper::rt;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    filename: Option<PathBuf>,
    #[structopt(short = "p", long = "base-port")]
    base_port: Option<u16>,
}

fn main() -> Result<(), Box<dyn Error>> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "crucio=info");
    }

    pretty_env_logger::init();

    let opt = Opt::from_args();

    let data = match opt.filename {
        Some(filename) => {
            let mut file = File::open(filename)?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)?;
            contents
        }
        None => include_bytes!("../index.txt").to_vec(),
    };

    let bind_addr: Ipv4Addr = [127, 0, 0, 1].into();
    let base_port = opt.base_port.unwrap_or(10000);

    rt::run(rt::lazy(move || {
        modes::spawn_all(&data, bind_addr, base_port);
        Ok(())
    }));

    Ok(())
}
