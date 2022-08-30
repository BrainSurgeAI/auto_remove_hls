use std::net::Ipv4Addr;
use std::ffi::CString;
use std::os::raw::c_char;
use postgres::{Client, NoTls,};
use std::path::PathBuf;
use clap::Parser;
use chrono::prelude::*;
use log::{error, info, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    filter::threshold::ThresholdFilter,
};
use log4rs::encode::pattern::PatternEncoder;


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
    log_path: PathBuf
}

struct Video {
    date: NaiveDate
}

fn main() -> Result<(), SetLoggerError> {
    let args = Args::parse();

    let level = log::LevelFilter::Info;
    let file_path = args.log_path;
    
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .build();
        
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} {l} {t} - {m}{n}")))
        .build(file_path)
        .unwrap();

    let config = Config::builder()
        .appender(
            Appender::builder()
                .build("logfile", Box::new(logfile)))
                .appender(
                    Appender::builder()
                        .filter(Box::new(ThresholdFilter::new(level)))
                        .build("stderr", Box::new(stderr)),
                )
                .build(
                    Root::builder()
                        .appender("logfile")
                        .appender("stderr")
                        .build(LevelFilter::Info),
                )
                .unwrap();
                    
    let _handler = log4rs::init_config(config)?;
      
    match Client::connect(
        format!("postgresql://postgres:postgres@{}/vod", args.db_ip).as_str(), 
        NoTls)
    {
        Ok(mut client) => {
            info!("Database connected");

            let path = args.videos_path.to_str().unwrap();
            let c = CString::new(path).unwrap();
            let c_path = c.as_ptr() as *const c_char;
                    
            unsafe {
                let mut res = has_enough_space(c_path, args.threshold);
                
                while !res {
                    info!("{:?} 可用空间不足{}%", path, args.threshold);

                    match client.query("SELECT min(date)::date as min_date from video", &[]){
                        Ok(rows) => {
                            for row in rows {
                                let video = Video {
                                    date: row.get(0)
                                };
                                
                                let mut date_string = video.date.to_string();
                                let condition = date_string.clone();
                                
                                date_string.remove(7);
                                let date = CString::new(date_string).unwrap();
                                let c_date = date.as_ptr() as *const c_char;
                
                                info!("Remove {} files", remove_files(c_path, c_date));
                                
                                let query = format!("DELETE FROM video where TO_CHAR(date::DATE, 'yyyy-mm-dd') = '{}'", condition);
                                match client.query(&query, &[]) {
                                    Ok(_) => info!("Delete records which date equals {}", condition),
                                    Err(e) => error!("Execute sql {} {}", &query, e)
                                }
                                
                            }
                        }
                        Err(err) => error!("{}", err)
                    }

                    res = has_enough_space(c_path, args.threshold);
                }
            }

            match client.close() {
                Ok(_) => info!("Database closed;"),
                Err(e) => error!("{}", e)
            }
        },
        Err(e) => error!("{}", e)
    }

    Ok(())
}
