#![allow(unused)]

use firebase_rs::*;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IResponse {
    name: String,
}

async fn set_user(fb_client: &Firebase, user: &User) -> IResponse {
    let firebase = fb_client.at("users");
    serde_json::from_str(firebase.set::<User>(user).await.unwrap().data.as_str()).unwrap()
}

async fn get_users(fb_client: &Firebase) -> HashMap<String, User> {
    let firebase = fb_client.at("users");
    firebase.get::<HashMap<String, User>>().await.unwrap()
}

async fn get_user(fb_client: &Firebase, id: &String) -> User {
    let firebase = fb_client.at("users").at(&id);
    firebase.get::<User>().await.unwrap()
}

async fn update_user(fb_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = fb_client.at("users").at(&id);
    serde_json::from_str(firebase.update::<User>(user).await.unwrap().data.as_str()).unwrap()
}

async fn delete_user(fb_client: &Firebase, id: &String) -> String {
    let firebase = fb_client.at("users").at(&id);
    firebase.delete().await.unwrap().data
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url = std::env::var("FIREBASE_URL").expect("FIREBASE_URL must be set.");

    let user = User {
        name: String::from("Mourad EL CADI"),
        age: 24,
        email: String::from("mourad@mail.com"),
    };

    let firebase = Firebase::new(&url).unwrap();

    let response = set_user(&firebase, &user).await;

    let id = &response.name;

    println!("[RESPONSE]: {:#?} \n\n", response);

    let mut user = get_user(&firebase, id).await;
    
    println!("[USER]: {:#?} \n\n", user);
    
    let users = get_users(&firebase).await;
    
    println!("[USERS]: {:#?}\n\n", users);

    user.email = String::from("some@mail.co");

    let updated_user = update_user(&firebase, id, &user).await;
    
    println!("[UPDATED USER]: {:#?}", updated_user);
    
    delete_user(&firebase, id).await;
}
