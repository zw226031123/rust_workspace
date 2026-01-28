use elasticsearch::auth::Credentials;
use elasticsearch::cat::CatIndicesParts;
use elasticsearch::http::Url;
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::{Elasticsearch, Error, IndexParts};
use serde_json::{Value, json};
#[tokio::main]
async fn main() {
    let url = Url::parse("http://127.0.0.1:9200").expect("Invalid Elasticsearch URL.");
    let conn_pool = SingleNodeConnectionPool::new(url);
    let credentials = Credentials::Basic("elastic".into(), "Pass@word1".into());
    let transport = TransportBuilder::new(conn_pool)
        .disable_proxy()
        .auth(credentials)
        .build()
        .expect("Failed to build the transport");
    let client = Elasticsearch::new(transport);
    save(&client).await.expect("save error");
    indices(&client).await.expect("indices error")
}
async fn save(client: &Elasticsearch) -> Result<(), Error> {
    client
        .index(IndexParts::IndexId("tweets", "1"))
        .body(json!({
            "user": "kimchy",
            "message": "Hello, Elasticsearch!"
        }))
        .send()
        .await?;
    Ok(())
}

async fn indices(client: &Elasticsearch) -> Result<(), Error> {
    let body = client
        .cat()
        .indices(CatIndicesParts::Index(&["*"]))
        .format("json")
        .send()
        .await?
        .json::<Value>()
        .await?;
    for idx in body.as_array().unwrap() {
        println!("Index: {}", idx["index"]);
    }
    Ok(())
}
