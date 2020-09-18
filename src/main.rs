use std::env;
use std::fs;
use std::path::Path;
use url::Url;
use std::fs::File;
use std::io::Write;
use std::result::Result;
use std::collections::HashMap;

mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [output_folder]", &args[0]);
        std::process::exit(1);
    }
    let output_folder = &args[1];
    if let Err(e) = fs::create_dir_all(&output_folder) {
        return Err(e.into());
    };

    let endpoint = env::var("ENDPOINT").expect("ENDPOINT env not provided.");

    let uri = Url::parse(&endpoint).expect(&format!("Invalid endpoint: {}", endpoint));

    let resp = reqwest::get(uri)
        .await?
        .text()
        .await?;

    let config: types::Config = serde_yaml::from_str(&resp).expect(&format!("Malformed response: {}", &resp));
    let proxies: Vec<types::Proxy> = serde_yaml::from_value(config.proxies).expect(&format!("Malformed proxy field: {}", &resp));

    let mut output = HashMap::new();
    output.insert("hk", Vec::new());
    output.insert("sg", Vec::new());
    output.insert("us", Vec::new());
    output.insert("jp", Vec::new());
    output.insert("cn", Vec::new());
    output.insert("all", Vec::new());

    for proxy in proxies.iter() {
        if proxy.name.contains("香港") || proxy.server.contains("hk") {
            output.get_mut("hk").unwrap().push(proxy);
        } else if proxy.name.contains("新加坡") || proxy.server.contains("sg") {
            output.get_mut("sg").unwrap().push(proxy);
        } else if proxy.name.contains("美国") || proxy.server.contains("us") {
            output.get_mut("us").unwrap().push(proxy);
        } else if proxy.name.contains("日本") || proxy.server.contains("jp") {
            output.get_mut("jp").unwrap().push(proxy);
        } else if proxy.name.contains("中国") || proxy.server.contains("cn") {
            output.get_mut("cn").unwrap().push(proxy);
        }

        output.get_mut("all").unwrap().push(proxy);
    }

    
    for (loc, proxies) in &output {
        let c = types::Config{
            proxies: serde_yaml::to_value(proxies).unwrap()
        };
        let output_path = format!("{}/{}.yml", output_folder, loc);
        let path = Path::new(&output_path);
        let mut file = match File::create(&path) {
            Err(why) => return Err(why.into()),
            Ok(file) => file,
        };
        if let Err(e) = file.write_all(serde_yaml::to_string(&c).unwrap().as_bytes()) {
            return Err(e.into());
        }
    };

    Ok(())
}
