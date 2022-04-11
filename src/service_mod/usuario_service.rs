use crate::data_access::postg;
use crate::data_access::postg::Usuario;

pub async fn login_service(email: String, senha:String) -> Usuario {

    let mut usuario: postg::Usuario = Usuario {id:0, nome:String::new(), email:String::new(), senha:String::new()};
    
    match postg::get_usuario(email, senha).await {
        Ok(u) => {usuario=u},
        Err(e) => println!("Erro = {}",e),
    }
    usuario    
} 

pub async fn get_all() -> Vec<Usuario> {
    
    let mut lista: Vec<Usuario> = Vec::new();

    match postg::get_all().await {
        Ok(u) => {lista=u},
        Err(e) => println!("Erro = {}",e),
    }
    lista
}

pub async fn get_by_id(id: i64) -> Usuario {
    let mut usuario: postg::Usuario = Usuario {id:0, nome:String::new(), email:String::new(), senha:String::new()};
    
    match postg::get_by_id(id).await {
        Ok(u) => {usuario=u},
        Err(e) => println!("Erro = {}",e),
    }
    usuario  
}

pub async fn insere_usuario_srv(usuario: &mut Usuario ) -> Usuario {
     match postg::insert_usuario(usuario).await {
        Ok(_u) => {
            return     Usuario {
                id: _u.id,
                nome: _u.nome.to_string(),
                email: _u.email.to_string(),
                senha: _u.senha.to_string(),
            }
        },
        Err(e) =>{ 
            println!("insere_usuario erro: {} ", e);
            return     Usuario {
                id: 0,
                nome: usuario.nome.to_string(),
                email: usuario.email.to_string(),
                senha: usuario.senha.to_string(),
            }
        },
    }
}