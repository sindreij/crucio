#![feature(async_await)]

mod modes;

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
    filename: PathBuf,
    #[structopt(short = "p", long = "base-port")]
    base_port: Option<u16>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut file = File::open(opt.filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let bind_addr: Ipv4Addr = [127, 0, 0, 1].into();
    let base_port = opt.base_port.unwrap_or(10000);

    rt::run(rt::lazy(move || {
        modes::spawn_all(&contents, bind_addr, base_port);
        Ok(())
    }));

    Ok(())
}
