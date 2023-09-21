use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn try_api() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000").unwrap();

    hc.do_post(
        "/update",
        json!({
            "userName": "tiendang",
            "userPassword": "password"
        }),
    )
    .await?
    .print()
    .await?;

    Ok(())
}
