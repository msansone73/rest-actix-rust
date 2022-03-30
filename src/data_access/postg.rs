use tokio_postgres::{NoTls, Error};

pub async fn insert_color(color: &str) -> Result<(), Error>{
    let str_conn = "postgresql://msansone:senha@localhost:5432/postgres";

    // Connect to the database.
    let (client, connection) =
        tokio_postgres::connect(&str_conn, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let _res = client.execute("INSERT INTO color(color_name) VALUES ($1);", &[&color]).await;

    Ok(())

}