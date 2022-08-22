use std::net::Ipv4Addr;
use std::ffi::CString;
use std::os::raw::c_char;
use postgres::{Client, NoTls, Error};
use std::path::PathBuf;
use clap::Parser;
use chrono::prelude::*;

extern "C" {
    /// Check available space of given path at least 20%
    fn has_enough_space(path: *const c_char, threshold: u8) -> bool;
    fn remove_files(dir: *const c_char, file_date: *const c_char) -> usize;
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// 待加密视频文件存储路径
    #[clap(parse(from_os_str))]
    videos_path: PathBuf,
    threshold: u8,
    db_ip: Ipv4Addr,
}

struct Video {
    date: NaiveDate
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
      
    let mut client = Client::connect(
        format!("postgresql://postgres:postgres@{}/vod", args.db_ip).as_str(), NoTls)?;
        
    let path = args.videos_path.to_str().unwrap();
    let c = CString::new(path).unwrap();
    let c_path = c.as_ptr() as *const c_char;
            
    unsafe {
        while !has_enough_space(c_path, args.threshold) {
            println!("has enough space");
            for row in client.query("SELECT min(date)::date as min_date from video", &[])? {
                let video = Video {
                    date: row.get(0)
                };
                
                let mut date_string = video.date.to_string();
                let condition = date_string.clone();
                
                date_string.remove(7);
                let date = CString::new(date_string).unwrap();
                let c_date = date.as_ptr() as *const c_char;

                if remove_files(c_path, c_date) > 0 {
                    client.query("DELETE FROM video where TO_CHAR(date::DATE, 'yyyy-mm-dd') = $1", &[&condition])?;
                }
            }
        }
    }
    
    Ok(())
}
