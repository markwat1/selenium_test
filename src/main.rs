use thirtyfour::prelude::*;
use tokio;
use webdriver_install::Driver;
use std::process::{Command};
use std::thread::JoinHandle;
use std::{thread,time};
use dirs;
use clap::Parser;
use std::sync::atomic::{AtomicBool,Ordering};

//use std::path::PathBuf;

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
async fn webdriver(browser:String,port:i32) -> WebDriverResult<()>{
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
    driver.quit().await?;
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    #[clap(short,long, value_parser, default_value = "Firefox")]
    browser: String,
}

fn main() -> WebDriverResult<()> {
    let args = Args::parse();
    static LIVE: AtomicBool = AtomicBool::new(true);
    let browser = args.browser;
    let port:i32 = 4444;
    let handler = webdriver_init(&browser,port,&LIVE);
    let result = webdriver(browser,port);
    LIVE.store(false,Ordering::Relaxed);
    handler.join().expect("Cannot join");
    result
}

