use thirtyfour::prelude::*;
use tokio;
use webdriver_install::Driver;
use std::{thread,time};
use std::thread::JoinHandle;
use std::process::{Command};
use std::sync::atomic::{AtomicBool,Ordering};
use dirs;
use clap::Parser;

mod sequence_parser;

fn webdriver_init(browser:&String, port: i32, live:&'static AtomicBool) -> JoinHandle<()>{
    let mut driver_name = "chromedriver";
    if browser == "Firefox" {
        driver_name = "geckodriver";
        Driver::Gecko.install().unwrap();
    }else{
        Driver::Chrome.install().unwrap();
    }
    let default_path = ".webdrivers";
    let home_path = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
    let driver_file = home_path + "/" + default_path + "/" + driver_name;
    let port_arg = format!("--port={}",port);
    thread::spawn(move ||{
        let mut webdriver  = Command::new(driver_file);
        webdriver.arg(port_arg);
        let mut child = webdriver.spawn().expect("failed to execute process");
        while live.load(Ordering::Relaxed) {
            thread::sleep(time::Duration::from_secs(1));
        }
        child.kill().expect("Kill error");
    })
}


#[tokio::main]

async fn webdriver(browser:String,port:i32,sequence_file:String) -> WebDriverResult<()>{
    let driver:WebDriver;
    let server_url = format!("http://localhost:{}",port);
    if browser == "Firefox" {
        driver = WebDriver::new(&server_url, DesiredCapabilities::firefox()).await?;
    }else{
        driver = WebDriver::new(&server_url, DesiredCapabilities::chrome()).await?;
    }
    // Navigate to URL.
    driver.get("http://www.watana.be/").await?;
    thread::sleep(time::Duration::from_secs(3));
    let v = sequence_parser::load_sequence(&sequence_file).unwrap();
    println!("{:?}",v);
    println!("Version : {}",sequence_parser::get_version(&v));
    println!("Comment : {}",sequence_parser::get_comment(&v));
    println!("url:{}",sequence_parser::get_url(&v));
    driver.quit().await?;
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    #[clap(short,long, value_parser, default_value = "Firefox")]
    browser: String,
    #[clap(short, long, value_parser, default_value = "sequence.json")]
    sequence: String
}

fn main() -> WebDriverResult<()> {
    let args = Args::parse();
    static LIVE: AtomicBool = AtomicBool::new(true);
    let browser = args.browser;
    let sequence_file = args.sequence;
    let port:i32 = 4444;
    let handler = webdriver_init(&browser,port,&LIVE);
    let result = webdriver(browser,port,sequence_file);
    LIVE.store(false,Ordering::Relaxed);
    handler.join().expect("Cannot join");
    result
}

