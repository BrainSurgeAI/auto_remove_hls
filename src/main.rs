use std::net::Ipv4Addr;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use postgres::{Client, NoTls, Error};
use std::path::PathBuf;
use clap::Parser;
use chrono::prelude::*;

extern "C" {
    fn has_enough_space(path: *const c_char) -> bool;
    fn remove_list(path: *const c_char) -> *const *const c_char;
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// 待加密视频文件存储路径
    #[clap(parse(from_os_str))]
    videos_path: PathBuf,

    db_ip: Ipv4Addr,
}

struct Video {
    date: NaiveDate
}

fn main() -> Result<(), Error> {
    let args = Args::parse();    
    let mut client = Client::connect(
        format!("postgresql://postgres:postgres@{}/vod", args.db_ip).as_str(),
        NoTls)?;
        
    let path = args.videos_path.to_str().unwrap();
    let c = CString::new(path).unwrap();
    let c_path = c.as_ptr() as *const c_char;
            
    unsafe {
        if has_enough_space(c_path) {
            for row in client.query("SELECT min(date)::date as min_date from video", &[])? {
                let video = Video {
                    date: row.get(0)
                };
                println!("{}", video.date);
            }
        }
    }
        
    Ok(())
}
