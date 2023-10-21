use std::env;
use anyhow::Result;
use serde_json::json;

struct CONFIG {
    api_url: String
}

impl CONFIG {
    fn new() -> Self {
        let port = env::var("DEVLOGS_AU_RESTFUL_PORT")
        .map(|env_var| env_var.parse().expect("The DEVLOGS_AU_RESTFUL_PORT must be number"))
            .unwrap_or(3000);

        Self {
            api_url: format!("http://localhost:{}", port)
        }
    }
}

#[tokio::test]
async fn create_user() -> Result<()> {
    let config = CONFIG::new();
    let hc = httpc_test::new_client(config.api_url).unwrap();

    hc.do_post(
        "/user",
        json!({
            "name": "tiendang",
            "fullName": "Dang Minh Tiến",
            "password": "password"
        }),
    )
    .await?
    .print()
    .await?;

    Ok(())
}
