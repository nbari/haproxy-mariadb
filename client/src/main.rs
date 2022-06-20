use mysql_async::prelude::*;

#[tokio::main]
async fn main() {
    let opts = mysql_async::OptsBuilder::default()
        .user(Some("root"))
        .pass(Some("secret"))
        .db_name(Some("world"))
        .ip_or_hostname(String::from("127.0.0.1"))
        .tcp_port(3306)
        .stmt_cache_size(0);
    let mut conn = mysql_async::Conn::new(opts).await.unwrap();

    let version: Option<String> = conn.query_first("SELECT VERSION()").await.unwrap();
    drop(conn);
    println!("{}", version.unwrap());
}
