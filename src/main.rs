mod common;
mod config;
mod honor;
mod huawei;

use crate::huawei::HuaweiConfig;
use ansi_term::Color::Green;
use clap::{Args, Parser, Subcommand};
use env_logger::WriteStyle;
use honor::{HonorConfig, MessageBuilder as HonorMessageBuilder};
use huawei::RequestMessageBuilder as HuaweiRequestMessageBuilder;
use log::{info, warn, LevelFilter};
use std::{env, fs};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// 应用相关
    #[clap(arg_required_else_help = true)]
    App {
        /// 配置文件初始化
        #[clap(short, long, action)]
        init: bool,
    },
    /// 推送通道相关
    #[clap(arg_required_else_help = true)]
    Channel {
        /// 指定通道创建请求参数文件: honor、huawei、oppo、vivo、xiaomi、meizu、apns、fcm
        #[clap(short, long, value_parser, name = "channel")]
        init: String,
    },
    /// 消息推送相关
    #[clap(arg_required_else_help = true)]
    Push(PushOptions),
}

#[derive(Args, Debug)]
struct PushOptions {
    /// 指定通道名: honor、huawei、oppo、vivo、xiaomi、meizu、apns、fcm
    #[clap(required = true, value_parser)]
    channel: String,
    /// 请求参数文件路径
    #[clap(short, long, value_parser)]
    file: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let cli = Cli::parse();
    match &cli.command {
        Commands::App { init: _init } => {
            info!("init dai config file");
            config::PushConfig::init();
        }
        Commands::Channel { init } => {
            create_message_file(init.as_str());
        }
        Commands::Push(push_options) => {
            let channel = push_options.channel.to_lowercase();
            let push_config = config::PushConfig::get_app_config();
            if push_config.is_none() {
                warn!("config file not found");
                return Ok(());
            }
            match channel.as_str() {
                "honor" => {
                    let config = push_config.unwrap().honor.unwrap();
                    honor_push(push_options, &config).await?;
                }
                "huawei" => {
                    let config = push_config.unwrap().huawei.unwrap();
                    huawei_push(push_options, &config).await?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

/// 生成下发文件
fn create_message_file(channel: &str) {
    let content = match channel {
        "honor" => {
            let message = HonorMessageBuilder::new().build();
            serde_json::to_string_pretty(&message).unwrap()
        }
        "huawei" => {
            let message = HuaweiRequestMessageBuilder::new().build();
            serde_json::to_string_pretty(&message).unwrap()
        }
        _ => "".to_string(),
    };

    let path = env::current_dir()
        .unwrap()
        .join(channel.to_owned() + ".json");
    fs::write(&path, &content).unwrap();
    info!(
        "create {} channel message template file: {}",
        channel,
        Green.paint(path.to_str().unwrap())
    );
}

/// push message
async fn honor_push(
    options: &PushOptions,
    config: &HonorConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = options.file.clone().unwrap_or_else(|| {
        env::current_dir()
            .unwrap()
            .join("honor.json")
            .to_str()
            .unwrap()
            .to_string()
    });
    info!("used file path: {}", file_path);
    let content = fs::read_to_string(file_path)?;
    let message = serde_json::from_str(&content)?;
    honor::send_message(config, &message).await?;
    Ok(())
}

/// push message
async fn huawei_push(
    options: &PushOptions,
    config: &HuaweiConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = options.file.clone().unwrap_or_else(|| {
        env::current_dir()
            .unwrap()
            .join("huawei.json")
            .to_str()
            .unwrap()
            .to_string()
    });
    info!("used file path: {}", file_path);
    let content = fs::read_to_string(file_path)?;
    let message = serde_json::from_str(&content)?;
    huawei::send_message(config, &message).await?;
    Ok(())
}

/// 初始化日志文件
fn init_logger() {
    env_logger::builder()
        .filter(None, LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .format_timestamp(None)
        .format_level(false)
        .format_module_path(false)
        .format_target(false)
        .init();
}
