use core::panic;
use std::env;

use tokio_postgres::{NoTls, Error, Client};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Usuario {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub nome: String,
    pub email: String,
    pub senha: String,
}

async fn get_db() -> String {
    let key = "Msansone_DB";

    match env::var_os(key) {
        Some(val) => return val.to_str().unwrap().to_string(),
        None => panic!("Msansone_DB not found ")
    }
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
    let mut fetch_usuario= Usuario {id:0,  nome: String::new(), email: String::new(), senha: String::new()};
    
    match get_usuario_by_email(&usuario.email).await {
        Ok(u) =>{fetch_usuario=u},
        Err(e) =>{println!("Erro {}",e)}
    }

    if !fetch_usuario.email.eq(""){
        return Ok(usuario)
    }

    let client = get_client().await;

    client.execute("insert into usuario (nome, email, senha) values ($1, $2, $3);", 
        &[&usuario.nome, &usuario.email, &usuario.senha])
        .await?;

    let mut id: i64=0;
    match get_usuario_by_email(&usuario.email).await {
        Ok(u) =>{
            id=u.id;
        },
        Err(e) =>{println!("Erro {}",e)}
    }   
    usuario.id=id;

    Ok(usuario)

}
