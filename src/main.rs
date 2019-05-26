#![feature(async_await)]

mod drop;
mod echo;
mod healthy;
mod never;
mod never_body;
mod random;
mod random_text;
mod slow;
mod slow_body;

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
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let mut file = File::open(opt.filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let bind_addr: Ipv4Addr = [127, 0, 0, 1].into();

    rt::run(rt::lazy(move || {
        rt::spawn(healthy::bind(&contents, (bind_addr, 3000)));
        rt::spawn(slow::bind(&contents, (bind_addr, 3001)));
        rt::spawn(slow_body::bind(&contents, (bind_addr, 3002)));
        rt::spawn(random::bind((bind_addr, 3003)));
        rt::spawn(random_text::bind((bind_addr, 3004)));
        rt::spawn(never::bind((bind_addr, 3005)));
        rt::spawn(never_body::bind((bind_addr, 3006)));
        rt::spawn(echo::bind((bind_addr, 3007)));
        rt::spawn(drop::bind((bind_addr, 3008)));

        Ok(())
    }));

    Ok(())
}
