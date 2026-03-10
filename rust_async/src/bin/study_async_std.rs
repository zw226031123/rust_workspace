use async_std::io::ReadExt;
use async_std::{net, task};
use futures::AsyncWriteExt;

fn main() -> std::io::Result<()> {
    // let response = task::block_on(cheapo_request("example.com", 80, "/"))?;
    // println!("{}", response);
    Ok(())
}
// async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
//     let mut socket = net::TcpStream::connect((host, port)).await;
//     let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
//     socket?.write_all(request.as_bytes()).await?;
//     socket?.shutdown(net::Shutdown::Write)?;
//     let mut response = String::new();
//     socket?.read_to_string(&mut response).await?;
//     Ok(response)
// }
// 5bc77727716dad6a5929243bfc3c5eae
// curl -X "POST"  "http://localhost:10001/v1/api/saas/mapping/syncPaasMappingByTenantId" -H 'Content-Type: application/json' -d '{"tenantId":"5bc77727716dad6a5929243bfc3c5eae"}'
//./flink run -m localhost:8081 -p 1 -c cn.publink.shb.sync.job.paas.PaasRunFormContentFlink /flink_share/flink-sync-1.5.0-SNAPSHOT.jar --env prod_multi --tenantId 5bc77727716dad6a5929243bfc3c5eae

