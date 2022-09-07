mod common;
mod config;
mod honor;
mod huawei;
mod xiaomi;

use crate::common::ChannelType;
use crate::config::PushConfig;

use ansi_term::Color::Green;
use clap::{Args, Parser, Subcommand};
use env_logger::WriteStyle;
use honor::MessageBuilder as HonorMessageBuilder;
use huawei::RequestMessageBuilder as HuaweiRequestMessageBuilder;
use log::{error, info, warn, LevelFilter};
use std::str::FromStr;
use std::{env, fs};
use xiaomi::MessageBuilder as XiaomiMessageBuilder;

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
            PushConfig::init();
        }
        Commands::Channel { init } => {
            let channel_type = ChannelType::from_str(&init.to_lowercase()).unwrap();
            create_message_file(&channel_type);
        }
        Commands::Push(push_options) => {
            let channel_type = ChannelType::from_str(&push_options.channel.to_lowercase()).unwrap();
            let push_config = PushConfig::get();
            if push_config.is_err() {
                error!("config file not found");
                return Ok(());
            }

            push_message(push_options, &channel_type, &push_config.unwrap()).await?;
        }
    }
    Ok(())
}

/// 生成下发文件
fn create_message_file(channel_type: &ChannelType) {
    let content = match channel_type {
        ChannelType::Huawei => {
            let message = HuaweiRequestMessageBuilder::new().build();
            serde_json::to_string_pretty(&message).unwrap()
        }
        ChannelType::Honor => {
            let message = HonorMessageBuilder::new().build();
            serde_json::to_string_pretty(&message).unwrap()
        }
        ChannelType::Xiaomi => {
            let message = XiaomiMessageBuilder::new().build();
            serde_json::to_string_pretty(&message).unwrap()
        }
        _ => "".to_string(),
    };

    let path = env::current_dir()
        .unwrap()
        .join(channel_type.to_string() + ".json");
    fs::write(&path, &content).unwrap();
    info!(
        "create {} channel message template file: {}",
        channel_type.to_string(),
        Green.paint(path.to_str().unwrap())
    );
}

/// 消息推送
async fn push_message(
    options: &PushOptions,
    channel_type: &ChannelType,
    push_config: &PushConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = options.file.clone().unwrap_or_else(|| {
        env::current_dir()
            .unwrap()
            .join(channel_type.to_string() + ".json")
            .to_str()
            .unwrap()
            .to_string()
    });
    info!("use file: {}", file_path);

    let content = fs::read_to_string(file_path).unwrap();

    let channel_config = PushConfig::get_channel_configs(&push_config);
    if channel_config.is_none() {
        warn!("current app have no channel config info!");
    }

    match channel_type {
        ChannelType::Huawei => {
            let message = serde_json::from_str(&content);
            let config = &channel_config.unwrap().huawei.unwrap();
            huawei::send_message(config, &message.unwrap()).await?;
        }
        ChannelType::Honor => {
            let message = serde_json::from_str(&content);
            let config = &channel_config.unwrap().honor.unwrap();
            honor::send_message(config, &message.unwrap()).await?;
        }
        ChannelType::Xiaomi => {
            let message = serde_json::from_str(&content);
            let config = &channel_config.unwrap().xiaomi.unwrap();
            xiaomi::send_message(config, &message.unwrap()).await?;
        }
        _ => {}
    }
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
