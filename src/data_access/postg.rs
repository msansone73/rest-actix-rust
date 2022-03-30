use tokio_postgres::{NoTls, Error};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Usuario {
    pub nome: String,
    pub email: String,
    pub senha: String,
}

pub async fn get_usuario(email: String, senha: String) -> Result<Usuario, Error> {
    let str_conn = "postgresql://msansone:sansone73@msansone.com.br:5432/sansone-fin";

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

        // Now we can execute a simple statement that just returns its parameter.
        let rows = client
        .query("select * from usuario where email =$1 and senha =$2;", &[&email, &senha])
        .await?;

        if rows.is_empty() {
            let usu:Usuario = Usuario { nome: String::new(), email: String::new(), senha: String::new()};
            Ok(usu)            
        } else {
            let _nome: &str = rows[0].get(1);
            let _email: &str = rows[0].get(2);
            let _senha: &str = rows[0].get(3);
            let usu:Usuario = Usuario { nome: _nome.to_string(), email: _email.to_string(), senha: _senha.to_string()};
            Ok(usu)
        }
}

pub async fn insert_color(color: &str) -> Result<(), Error>{
    let str_conn = "postgresql://msansone:sansone73@msansone.com.br:5432/sansone-fin";

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