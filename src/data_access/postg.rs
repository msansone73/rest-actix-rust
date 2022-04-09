use core::panic;

use tokio_postgres::{NoTls, Error, Client};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Usuario {
    pub id: i64,
    pub nome: String,
    pub email: String,
    pub senha: String,
}

async fn get_db() -> String {
    "postgresql://msansone:sansone73@msansone.com.br:5432/sansone-fin".to_string()
}

async fn get_client() -> Client  {
        let str_conn = get_db().await;
    
        let client;
        let connection;
        let res = tokio_postgres::connect(&str_conn, NoTls).await;
        match res {
            Ok((c,con))=> {
                client=c;
                connection=con;
                    tokio::spawn(async move {
                        if let Err(e) = connection.await {
                            eprintln!("connection error: {}", e);
                        }
                    });
                return client
            },
            Err(e) =>{
                panic!("DB access error: {}",e)
            }
        }

}

async fn get_usuario_by_email(email: &String) -> Result<Usuario, Error> {

    let client = get_client().await;

    let rows = client
        .query("select * from usuario where email =$1;", &[&email])
        .await?;

    if rows.is_empty() {
        let usu:Usuario = Usuario {id:0,  nome: String::new(), email: String::new(), senha: String::new()};
        Ok(usu)            
    } else {
        let _id=     rows[0].get::<_, i64>("id");
        let _nome: &str = rows[0].get("nome");
        let _email: &str = rows[0].get("email");
        let _senha: &str = rows[0].get("senha");
        let usu:Usuario = Usuario {id:_id,  nome: _nome.to_string(), email: _email.to_string(), senha: _senha.to_string()};
        Ok(usu)
    }
}

pub async fn get_usuario(email: String, senha: String) -> Result<Usuario, Error> {
    let client = get_client().await;

    let rows = client
        .query("select * from usuario where email =$1 and senha =$2;", &[&email, &senha])
        .await?;

    if rows.is_empty() {
        let usu:Usuario = Usuario {id:0,  nome: String::new(), email: String::new(), senha: String::new()};
        Ok(usu)            
    } else {
        let _id=     rows[0].get::<_, i64>("id");
        let _nome: &str =  rows[0].get("nome");
        let _email: &str = rows[0].get("email");
        let _senha: &str = rows[0].get("senha");
        let usu:Usuario = Usuario {id:_id,  nome: _nome.to_string(), email: _email.to_string(), senha: _senha.to_string()};
        Ok(usu)
    }
}

pub async fn insert_usuario(usuario: &mut Usuario) -> Result<&Usuario, Error> {
    println!("insert_usuario... ");
    let mut fetch_usuario= Usuario {id:0,  nome: String::new(), email: String::new(), senha: String::new()};
    
    match get_usuario_by_email(&usuario.email).await {
        Ok(u) =>{fetch_usuario=u},
        Err(e) =>{println!("Erro {}",e)}
    }

    if !fetch_usuario.email.eq(""){
        println!("Email {} already exists.", fetch_usuario.email);
        return Ok(usuario)
    }

    let client = get_client().await;

    client.execute("insert into usuario (nome, email, senha) values ($1, $2, $3);", 
        &[&usuario.nome, &usuario.email, &usuario.senha])
        .await?;

    let mut id: i64=0;
    match get_usuario_by_email(&usuario.email).await {
        Ok(u) =>{
            println!("insert_usuario... achei id={}", u.id);
            id=u.id;
        },
        Err(e) =>{println!("Erro {}",e)}
    }    
    println!("insert_usuario... id={}",id);

    usuario.id=id;

    Ok(usuario)

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