use argh::FromArgs;
use serde::{Serialize, Deserialize};

#[derive(FromArgs, Debug, Clone, Serialize, Deserialize)]
/// 简单的网页跳转服务
pub struct Args {
    /// html文件目录。默认读取index.html作为主页，jump.html作为跳转页面
    #[argh(option)]
    pub www: String,

    /// 是否开启静态文件服务。%www%/static
    #[argh(switch, short = 's')]
    pub static_file: bool,

    /// 自定义路由。如路由不含有target，则从query字符串里寻找target。
    #[argh(option, default = r#"String::from("/{target}")"#)]
    pub route: String,

    /// 监听地址，默认0.0.0.0
    #[argh(option, default = r#"String::from("0.0.0.0")"#, short = 'a')]
    pub bind: String,

    /// 监听端口
    #[argh(option, default = "7070u16", short = 'p')]
    pub port: u16,

    /// 安静模式，不输出日志
    #[argh(switch, short = 'q')]
    pub quiet: bool,

    /// 记录日志到文件，而不是只输出到控制台
    #[argh(switch, short = 'l')]
    pub log_to_file: bool,

    /// `不`将http重定向到https并监听80端口。
    #[argh(switch)]
    pub no_redirect: bool,

    /// ssl私匙文件
    #[argh(option)]
    pub ssl_key: Option<String>,

    /// ssl证书链文件
    #[argh(option)]
    pub ssl_cert: Option<String>
}