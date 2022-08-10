use std::net::Ipv4Addr;
use std::ffi::CString;
use std::os::raw::c_char;
use postgres::{Client, NoTls};
use std::path::PathBuf;
use clap::Parser;

extern "C" {
    fn has_enough_space(path: *const c_char) -> bool;
    
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// 待加密视频文件存储路径
    #[clap(parse(from_os_str))]
    videos_path: PathBuf,

    /// 数据库IP地址
    db_ip: Ipv4Addr,
}

fn main() {
    let args = Args::parse();
    
    match Client::connect(
        format!("postgresql://postgres:postgres@{}/vod", args.db_ip).as_str(),
        NoTls,
    ){
        Ok(mut client) => {
            let path = args.videos_path.to_str().unwrap();
            let c = CString::new(path).unwrap();
            let c_path = c.as_ptr() as *const c_char;
            
            unsafe {
                println!("{}", has_enough_space(c_path));
            }
        }
        Err(e) => println!("Can not connect to database; {}", e),
    };


}
