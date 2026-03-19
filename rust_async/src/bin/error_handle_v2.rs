use futures::future::join_all;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let _=fetch_user(3).await?;
    let mut tasks = Vec::new();
    for id in 1..10 {
        tasks.push(fetch_user(id));
    }
    let results = join_all(tasks).await;
    println!("{:?}", results);
    // let final_result :anyhow::Result<Vec<String>>= results.into_iter().collect();
    // let x:Vec<String> =final_result?;
    // println!("{:?}", x);
    // Ok(())

    let mut errors = Vec::new();
    let oks: Vec<String> = results
        .into_iter()
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("oks: {:?}", oks);
    println!("errors: {:?}", errors);
    Ok(())
}
async fn fetch_user(id: u32) -> anyhow::Result<String> {
    if id == 3 {
        anyhow::bail!("User {id} is not exists")
    }
    sleep(Duration::new(0, 1000)).await;
    anyhow::Ok(format!("User_{id}"))
}
