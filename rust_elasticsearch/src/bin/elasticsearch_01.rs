use elasticsearch::auth::Credentials;
use elasticsearch::http::Url;
use elasticsearch::http::response::Response;
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::{Elasticsearch, SearchParts};
use serde_json::{Value, json};
use tokio::runtime::Runtime;

fn main() {
    let url = Url::parse("http://172.16.0.118:9200").expect("Invalid Elasticsearch URL.");
    let conn_pool = SingleNodeConnectionPool::new(url);
    let credentials = Credentials::Basic("elastic".into(), "Pass@word1".into());
    let transport = TransportBuilder::new(conn_pool)
        .disable_proxy()
        .auth(credentials)
        .build()
        .expect("Failed to build the transport");
    let client = Elasticsearch::new(transport);
    let rt = Runtime::new().unwrap();
    let response = search(&rt, &client);
    let response_body = json(&rt, response);
    let took = response_body["took"].as_i64().unwrap();
    for hit in response_body["hits"]["hits"].as_array().unwrap() {
        // print the source document
        println!("{:?}", hit["_source"]);
    }
}

fn search(rt: &Runtime, client: &Elasticsearch) -> Response {
    rt.block_on(async {
        return client
            .search(SearchParts::Index(&[""]))
            .from(0)
            .size(10)
            .body(json!({
                "query": {
                    "match": {
                        "message": "Elasticsearch rust"
                    }
                }
            }))
            .send()
            .await
            .expect("search error");
    })
}

fn json(rt: &Runtime, response: Response) -> Value {
    rt.block_on(async move {
        return response.json::<Value>().await.expect("json error");
    })
}
