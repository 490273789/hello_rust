use anyhow::{anyhow, Ok, Result};
use clap::{Args, Parser, Subcommand};
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};

// 定义HTTPie的CLI主入口，包含若干个子命令
// 三条斜线///是文档注释，clap会将其作为CLI的帮助输出
/// A naive http_ie implementation with Rust, can you imagine how easy it is?

// #[derive(Parser)] 是一个过程宏（procedural macro），用于自动为结构体实现 clap::Parser trait。这使得该结构体可以用来解析命令行参数。
#[derive(Parser, Debug)]
#[command(version, author, about)]
struct HttpIe {
    #[command(subcommand)]
    methods: Method,
}

/// 子命令分别对应不同的HTTP方法，目前只支持get / post
#[derive(Subcommand, Debug)]
enum Method {
    Get(Get),
    Post(Post),
}

/// get子命令
#[derive(Args, Debug)]
struct Get {
    #[arg(value_parser = parse_url)]
    url: String,
}

/// post子命令。需要输入一个URL，和若干个可选的key=value pairs
#[derive(Args, Debug)]
struct Post {
    /// Specify the url you wanna request to.
    #[arg(value_parser = parse_url)]
    url: String,
    /// http 请求 body
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Debug, PartialEq, Clone)]
struct KvPair {
    k: String,
    v: String,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行split， 这会得到一个迭代器
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse {s}"));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

// 因为为KvPair 实现了 FormStr，这里可以通过s.parse() 得到KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

// 打印服务器版本号 + 状态吗
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{} : {:?}", name.to_string().green(), value);
    }
    println!("\n");
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        _ => println!("{}", body),
    }
}

async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[tokio::main]
async fn main() -> Result<()> {
    let http_ie: HttpIe = HttpIe::parse();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust HttpIe".parse()?);

    // 生成一个http客户端
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let result = match http_ie.methods {
        Method::Get(ref args) => get(client, args).await?,
        Method::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
