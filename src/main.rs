use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(short, long)]
    app: Option<String>,
    #[structopt(short, long)]
    env: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Items {
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    name: String,
    lastRequested: Option<String>,
    _links: Links,
}

impl Item {
    fn flag(&self) -> &str {
        self._links._self.href.split("/").nth(6).unwrap_or_default()
    }
}

#[derive(Debug, Deserialize)]
struct Links {
    #[serde(rename = "self")]
    _self: Link,
}

#[derive(Debug, Deserialize)]
struct Link {
    href: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let Opts { app, env } = Opts::from_args();
    for item in Client::new()
        .get(&format!(
            "https://app.launchdarkly.com/api/v2/flag-statuses/{}/{}",
            app.unwrap_or("default".into()),
            env.unwrap_or("production".into())
        ))
        .header("Authorization", env!("LD_API_KEY"))
        .send()
        .await?
        .json::<Items>()
        .await?
        .items
        .iter()
        .filter(|item| item.name == "inactive")
    {
        println!("{} {}", item.flag(), item.lastRequested.as_ref().unwrap_or(&"(never)".into()));
    }
    Ok(())
}
