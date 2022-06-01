use actix_web::web;
use actix_web::Responder;
use base32::encode;
use datamatrix::{data::EncodationType, DataMatrixBuilder};
use hex_literal::hex;
use image::{GrayImage, Luma};
use p256::ecdsa::{signature::Signer, SigningKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use reqwest::blocking::Client;
use std::fs::File;
use std::string::String;
use std::io::Read;
use std::io::Write;


use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub last_name: String,
    pub first_name: String,
    pub genre: String,
    pub birth_date: String,
    pub birth_country: String,
    pub diploma_degree: String,
    pub diploma_domain: String,
    pub diploma_mention: String,
    pub diploma_speciality: String,
    pub diploma_type: String,
}
pub async fn test() -> impl Responder {
	format!("2D-Doc crée")
}



pub async fn create_2ddoc(item: web::Json<InputUser>) -> impl Responder {
    let first_name = &item.first_name;
    let last_name = &item.last_name;
    let diploma_domain = &item.diploma_domain;
    let diploma_speciality = &item.diploma_speciality;
    let birth_date = &item.birth_date;
    let birth_country = &item.birth_country;
    let diploma_degree = &item.diploma_degree;
    let diploma_type = &item.diploma_type;
    let genre = &item.genre;
    let entete = "DC04FR0000011FF41FF4B001FR";
    let mut message = createdata(
        entete,
        last_name,
        first_name,
        diploma_domain,
        "",
        diploma_speciality,
        genre,
        birth_date,
        birth_country,
        diploma_degree,
        diploma_type,
    );
    let hash = sha2(&message);
    let sign = p256(&hash);
    message = message + "<US>" + &sign[1..sign.len() - 1];
    println!("message  :{}",message);
    create2ddoc(message);
    let image2=convert_file();
    let mut image3 = unsafe {
    String::from_utf8_unchecked(image2)
};
    rebuild_file(image3.as_bytes().to_vec());
    format!("{}",image3)
}

pub fn create2ddoc(message: String) {
    const N: usize = 5;

    // Encode the message using the smallest square it can fit into

    let bitmap = DataMatrixBuilder::new()
        .with_encodation_types(EncodationType::C40)
        .encode(message.as_bytes())
        .unwrap();

    // Create an image which only contains the Data Matrix including a quiet zone
    let width = ((bitmap.bitmap().width() + 2) * N) as u32;
    let height = ((bitmap.bitmap().height() + 2) * N) as u32;
    let mut image = GrayImage::from_pixel(width, height, Luma([255]));
    for (x, y) in bitmap.bitmap().pixels() {
        // Write the black square at x, y using NxN black pixels
        for i in 0..N {
            for j in 0..N {
                let x_i = (x + 1) * N + j;
                let y_j = (y + 1) * N + i;
                image.put_pixel(x_i as u32, y_j as u32, Luma([0]));
            }
        }
    }
    
    image.save("2DDOC.png").unwrap();
    
}


pub fn createdata(
    entete: &str,
    first_name: &str,
    last_name: &str,
    diploma_domain: &str,
    diploma_mention: &str,
    diploma_speciality: &str,
    genre: &str,
    birth_date: &str,
    birth_country: &str,
    diploma_degree: &str,
    diploma_type: &str,
) -> String {
    let mut first_name = first_name.to_owned();
    let mut last_name = last_name.to_owned();
    let mut diploma_domain = diploma_domain.to_owned();
    let mut diploma_mention = diploma_mention.to_owned();
    let mut diploma_speciality = diploma_speciality.to_owned();
    for _i in 0..20 - first_name.len() {
        first_name = first_name + " ";
    }
    for _i in 0..38 - last_name.len() {
        last_name = last_name + " ";
    }
    for _i in 0..30 - diploma_domain.len() {
        diploma_domain = diploma_domain + " ";
    }
    for _i in 0..30 - diploma_mention.len() {
        diploma_mention = diploma_mention + " ";
    }
    for _i in 0..30 - diploma_speciality.len() {
        diploma_speciality = diploma_speciality + " ";
    }
    let message = entete.to_owned()
        + "B1"
        + &first_name
        + "B2"
        + &last_name
        + "B6"
        + &genre
        + "B7"
        + &birth_date
        + "B9"
        + &birth_country
        + "BD"
        + &diploma_degree
        + "BG"
        + &diploma_type
        + "BH"
        + &diploma_domain
        + "BI"
        + &diploma_mention
        + "BJ"
        + &diploma_speciality
        + "0A111111111";
    return message;
}

pub fn sha2(message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());

    let hashed = hasher.finalize();

    return format!("{:x}", hashed);
}

pub fn p256(message: &str) -> String {
    // Signing
    //let key="MHcCAQEEINbI/xP+yGOgp79v7qibvYs03x+cSIaiKzpOhJsScwDDoAoGCCqGSM49AwEHoUQDQgAEqY8NfM1igIiTvsTUNuedGDSh1uAB1w8cTNzNnZ4v4in3JAUU6N3AypjQx0QMnMSShJoPvac/w5L02grgf4TCPA==".into_bytes();
    let key = &hex!("d0db9683bbe181ecfb323b1d4f259bd15d18fde9bcc06ed3e655ac6094856558");
    let signing_key = SigningKey::from_bytes(key).unwrap();
    let signature = signing_key.sign(message.as_bytes());
    let signb32 = encode(
        base32::Alphabet::RFC4648 { padding: true },
        signature.as_ref(),
    );
    let sign = &signb32[..signb32.len() - 1];
    
    return format!("{:?}", sign);
}
pub fn rebuild_file(data: Vec<u8>) {
    // path
    let mut path = "/home/thomas/Documents/2ddoc/test.png";
    let file_name = "test.png";
    // create empty file
    let mut new_file = File::create(path).unwrap();
    // put and write data (vec of byte) in file
    new_file.write(&data).unwrap();
    println!("this file: {:?}", new_file);
}
pub fn convert_file() -> Vec<u8> {
    let file_path = "/home/thomas/Documents/2ddoc/2DDOC.png";
    println!("path = {}", file_path);
    let mut file = File::open(file_path).unwrap();
    let mut contents: Vec<u8> = vec![];
    file.read_to_end(&mut contents).unwrap();
    println!("file convert to vec of byte");
    return contents;
}