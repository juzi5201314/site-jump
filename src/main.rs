#[macro_use]
extern crate log;

use std::ops::Add;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::error::ErrorInternalServerError;
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use anyhow::Result;
use colored::{Color, Colorize};
use fern::colors::ColoredLevelConfig;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::Deserialize;
use tera::{Context, Tera};

use crate::args::Args;

mod args;

#[actix_web::main]
async fn main() -> Result<()> {
    let args: Args = argh::from_env();

    if !args.quiet {
        init_log(args.log_to_file)?;
    }

    info!("Start parameter: {}", std::env::args().skip(1).collect::<Vec<String>>().join(" "));

    let addr = format!("{}:{}", args.bind, args.port);
    let temp_dir = args.www.clone();
    let use_static_file = args.static_file;
    let route = args.route.clone();
    let use_tsl = args.ssl_key.as_ref().and(args.ssl_cert.clone()).is_some();
    let no_redirect = args.no_redirect;

    // 克隆一份参数到actix web里
    let args_c = args.clone();
    let mut server = HttpServer::new(move || {
        let mut app = App::new()
            .wrap(RedirectSchemeBuilder::new().enable(!no_redirect && use_tsl).build())
            .route("/", web::get().to(index))
            .route(&route.clone(), web::get().to(handle))
            .data({
                let mut tera = Tera::new(&temp_dir.clone().add("/*.html")).expect("初始化Tera失败");
                tera.autoescape_on(Vec::new());
                tera
            })
            .data(args_c.clone());

        if use_static_file {
            app = app.service(actix_files::Files::new("/static", &temp_dir.clone().add("/static")))
        }

        app
    });

    // 使用https
    if use_tsl {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
        builder.set_private_key_file(&args.ssl_key.unwrap(), SslFiletype::PEM)?;
        builder.set_certificate_chain_file(&args.ssl_cert.unwrap())?;
        if !no_redirect {
            server = server.bind(format!("{}:80", &args.bind))?;
        }
        server = server.bind_openssl(&addr, builder)?;
        info!("Listen on https://{}", addr);
    } else {
        server = server.bind(&addr)?;
        info!("Listen on http://{}", addr);
    }

    server.run().await?;
    Ok(())
}

fn index(tera: web::Data<Tera>, args: web::Data<Args>) -> HttpResponse {
    let mut context = Context::new();
    context.insert("args", args.get_ref());

    let html = tera.render("index.html", &context).map_err(|err| ErrorInternalServerError(err));
    match html {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(err) => {
            error!("渲染index模板时出现错误: {}", err.to_string());
            err.into()
        }
    }
}

#[derive(Deserialize)]
struct QueryS {
    pub target: Option<String>
}

fn handle(req: HttpRequest, tera: web::Data<Tera>, args: web::Data<Args>, query: web::Query<QueryS>) -> HttpResponse {
    let target = req.match_info().get("target").map(|s| s.to_owned()).or(query.target.clone());

    let context_target = &urldecode(target.as_ref().unwrap_or(&"null".to_owned()));
    let mut context = Context::new();
    context.insert("legal", &url::Url::parse(context_target).is_ok());
    context.insert("target", context_target);
    context.insert("args", args.get_ref());

    let html = tera.render("jump.html", &context);
    match html {
        Ok(html) => {
            target.map(|target| info!("go to {}", target));
            HttpResponse::Ok().content_type("text/html").body(html)
        },
        Err(err) => {
            error!("渲染jump模板时出现错误: {}", err.to_string());
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

fn init_log(log_to_file: bool) -> Result<()> {
    let colors = ColoredLevelConfig::new()
        .info(fern::colors::Color::Green)
        .warn(fern::colors::Color::Yellow);

    let mut dispath = fern::Dispatch::new()
        .format(move |out, message, record|
            out.finish(format_args!(
                "[{time}] [{level}]: {message} <{target}>",
                time = chrono::Local::now().format("%y-%m-%d %H:%M:%S"),
                level = colors.color(record.level()),
                message = message,
                target = record.target().color(Color::Blue)
            ))
        )
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout());

    if log_to_file {
        dispath = dispath.chain(fern::log_file("site-jump.log")?);
    }
    dispath.apply()?;
    Ok(())
}

fn urldecode(url: &str) -> String {
    url::form_urlencoded::parse(url.as_bytes()).map(|(key, val)| [key, val].concat()).collect::<String>()
}