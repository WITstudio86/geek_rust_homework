use crate::{read_input, verify_file, CmdExector};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Claims {
    pub sub: String,
    pub aud: String,
    pub exp: usize,
}

pub struct JwtSign {
    pub key: Vec<u8>,
}

#[derive(Debug, Subcommand)]
#[enum_dispatch]
pub enum JwtSignCmd {
    #[command(about = "Sign jwt")]
    Sign(JwtSignArgs),
    #[command(about = "Verify jwt")]
    Verify(JwtVerifyArgs),
}
#[derive(Debug, Parser)]
pub struct JwtSignArgs {
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long,value_parser=convert_to_usize)]
    pub exp: usize,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyArgs {
    #[arg(short, long, value_parser = verify_file, default_value = "_")]
    pub token: String,
}

impl CmdExector for JwtSignCmd {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            JwtSignCmd::Sign(opts) => {
                let jwt = JwtSign::new();
                let claims = Claims {
                    sub: opts.sub,
                    aud: opts.aud,
                    exp: opts.exp,
                };
                let token = jwt.sign(claims.clone()).unwrap();
                println!("{}", String::from_utf8(token)?);
                Ok(())
            }
            JwtSignCmd::Verify(opts) => {
                let jwt = JwtSign::new();
                let token = read_input(&opts.token)?.trim_ascii().to_vec();
                let (header, readde_claims) = jwt.verify(token).unwrap();
                println!("header: \n{:#?}", header);
                println!("Data: \n{:#?}", readde_claims);
                Ok(())
            }
        }
    }
}

// 将输入的 exp 参数转换为实际有效的秒
fn convert_to_usize(value: &str) -> anyhow::Result<usize> {
    // 取出输入值中是数字的部分
    let number = value[..value.len() - 1].to_string().parse::<usize>()?;

    let symbol = &value[value.len() - 1..]; // 取出剩余部分

    let result = match symbol {
        "s" => number,
        "m" => number * 60,
        "h" => number * 60 * 60,
        "d" => number * 60 * 60 * 24,
        "w" => number * 60 * 60 * 24 * 7,
        _ => anyhow::bail!("Invalid time unit: {}", symbol),
    };
    Ok(result * 1000 + chrono::Utc::now().timestamp_millis() as usize)
}
