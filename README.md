# site-jump

## Usage
```
Usage: site-jump --www <www> [-s] [--route <route>] [-a <bind>] [-p <port>] [-q] [-l]

简单的网页跳转服务

Options:
  --www             html文件目录。默认读取index.html作为主页，jump.html作为跳转页面
  -s, --static-file 是否开启静态文件服务。%www%/static
  --route           自定义路由。如路由不含有target，则从query字符串里寻找target。
  -a, --bind        监听地址，默认0.0.0.0
  -p, --port        监听端口
  -q, --quiet       安静模式，不输出日志
  -l, --log-to-file 记录日志到文件，而不是只输出到控制台
  --help            display usage information

```

## Examples
```
site-jump --www "/home/me/www" -s --route "/{target}" -a 0.0.0.0 -p 7070 -l
```
html文件目录在"/home/me/www"，并启用静态文件服务"/home/me/www/static"。

路径为http://127.0.0.0:7070/http%3a%2f%2fgoogle.com。

并将日志记录到site-jump.log文件里。

## Build
#### Required
* [Rust](https://www.rust-lang.org/)

```
cargo build --release
```

## Development
模板引擎使用[tera](https://tera.netlify.app/docs) ，具体使用方法请查看tera的文档。

#### jump.html
对于跳转页面，目前提供了以下属性:
* legal: bool (表示目标url是否有效
* target: string (已urldecode的目标url



#### index.html
暂无

---
以上模板皆提供了一个args属性:
* args: Args (表示传入的命令行参数，具体查看src/args.rs