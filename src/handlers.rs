use actix_web::Responder;
use actix_web::{web};
use serde::{Deserialize, Serialize};
use datamatrix::DataMatrix;
use datamatrix::SymbolList;
use sha2::{Sha256, Digest};
use p256::{ecdsa::{SigningKey, signature::Signer},};
use hex_literal::hex;
use base32::encode;
use image::{GrayImage, Luma};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub nom: String,
    pub prenom: String,
    pub genre: String,
    pub date_naissance: String,
    pub pays_naissance: String,
    pub niveau_diplome: String,
    pub domaine: String,
    pub mention: String,
    pub spe: String,
    pub type_diplome: String,
}

pub async fn create_2ddoc(item: web::Json<InputUser>) -> impl Responder {
    let prenom= &item.prenom;
    let nom= &item.nom;
    let domaine= &item.domaine;
    let spe= &item.spe;
    let date_naissance=&item.date_naissance;
    let pays_naissance=&item.pays_naissance;
    let niveau_diplome=&item.niveau_diplome;
    let type_diplome=&item.type_diplome;
    let genre=&item.genre;
    let entete = "DC04FR000001111E1FC5B001FR";
    let mut message = createdata(entete,nom,prenom,domaine,"",spe,genre,date_naissance,pays_naissance,niveau_diplome,type_diplome);
    let hash=sha2(&message);
    let sign=p256(&hash);
    message = message + "<US>" + &sign[1..sign.len()-1];
    println!("Message={}",message);
    create2ddoc(message);
    format!("2D-Doc crée")
}




pub fn create2ddoc(message: String){
    const N: usize = 5;

    // Encode the message using the smallest square it can fit into
    let bitmap = DataMatrix::encode(message.as_bytes(), SymbolList::default().enforce_square())
        .unwrap()
        .bitmap();

    // Create an image which only contains the Data Matrix including a quiet zone
    let width = ((bitmap.width() + 2) * N) as u32;
    let height = ((bitmap.height() + 2) * N) as u32;
    let mut image = GrayImage::from_pixel(width, height, Luma([255]));
    for (x, y) in bitmap.pixels() {
        // Write the black square at x, y using NxN black pixels
        for i in 0..N {
            for j in 0..N {
                let x_i = (x + 1) * N + j;
                let y_j = (y + 1) * N + i;
                image.put_pixel(x_i as u32, y_j as u32, Luma([0]));
            }
        }
    }

    image.save("2D-DOC.png").unwrap();
}


pub fn createdata(entete: &str,prenom: &str,nom: &str,domaine: &str,mention: &str,spe: &str,genre: &str,date_naissance: &str,pays_naissance: &str,niveau_diplome: &str,type_diplome: &str) -> String {
    let mut prenom=prenom.to_owned();
    let mut nom=nom.to_owned();
    let mut domaine=domaine.to_owned();
    let mut mention=mention.to_owned();
    let mut spe=spe.to_owned();
    for _i in 0..20-prenom.len() {prenom=prenom + " ";}
    for _i in 0..38-nom.len() {nom=nom+" ";}
    for _i in 0..30-domaine.len() {domaine=domaine + " ";}
    for _i in 0..30-mention.len() {mention=mention + " ";}
    for _i in 0..30-spe.len() {spe=spe + " ";}
    let message = entete.to_owned() + "B1"+ &prenom + "B2" + &nom + "B6" + &genre + "B7" + &date_naissance + "B9" +&pays_naissance + "BD" + &niveau_diplome + "BG" + &type_diplome + "BH" +&domaine +"BI" + &mention + "BJ" +&spe + "0A111111111";
    return message
}


pub fn sha2(message: &str) -> String {

    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    
    let hashed =hasher.finalize();

    return format!("{:x}", hashed);

}


pub fn p256(message: &str) -> String {	
	// Signing
    //let key="MHcCAQEEINbI/xP+yGOgp79v7qibvYs03x+cSIaiKzpOhJsScwDDoAoGCCqGSM49AwEHoUQDQgAEqY8NfM1igIiTvsTUNuedGDSh1uAB1w8cTNzNnZ4v4in3JAUU6N3AypjQx0QMnMSShJoPvac/w5L02grgf4TCPA==".into_bytes();
    let key = &hex!("d0db9683bbe181ecfb323b1d4f259bd15d18fde9bcc06ed3e655ac6094856558");
    let signing_key= SigningKey::from_bytes(key).unwrap();
    let signature = signing_key.sign(message.as_bytes());
    let signb32 = encode(base32::Alphabet::RFC4648 { padding: true },signature.as_ref());
    let sign = &signb32[..signb32.len()-1];
	return format!("{:?}", sign);
}
