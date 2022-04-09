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