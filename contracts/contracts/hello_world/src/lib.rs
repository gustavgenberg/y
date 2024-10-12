#![no_std]
use core::fmt::{Debug, Formatter};

use soroban_sdk::{contract, contractimpl, contracttype, crypto, symbol_short, vec, Address, BytesN, Env, FromVal, String, Symbol, Vec};

#[contract]
pub struct YContract;

#[contracttype]
pub enum YeetKey {
    Of(String)
}

#[contracttype]
pub struct Yeet {
    message: String,
    author: Address,
    likes: u64,
    replies: Vec<Yeet>
}

#[contractimpl]
impl YContract {
    pub fn yeet(env: Env, user: Address, message: String, id: String, initial_validity: u32) {
        user.require_auth();

        let yeet_key = YeetKey::Of(id.clone());

        env.storage().temporary().set(&yeet_key, &Yeet {
            message: message,
            author: user.clone(),
            likes: 0,
            replies: Vec::new(&env)
        });

        env.storage().temporary().extend_ttl(&yeet_key, initial_validity, initial_validity);
    }

    pub fn reply(env: Env, user: Address, reply: String, id: String, added_validity: u32) {
        user.require_auth();

        let yeet_key = YeetKey::Of(id.clone());

        let mut root_yeet: Yeet = env.storage().temporary().get(&yeet_key).expect("This fucking yeet doesnt exist");

        let reply_yeet: Vec<Yeet> = vec![&env, Yeet {
            message: reply,
            author: user.clone(),
            likes: 0,
            replies: Vec::new(&env),
        }];

        root_yeet.replies.append(&reply_yeet);

        env.storage().temporary().set(&yeet_key, &root_yeet);

        env.storage().temporary().extend_ttl(&yeet_key, added_validity, added_validity);

    }

    pub fn sheesh(env: Env, user: Address, id: String, added_validity: u32) {
        user.require_auth();

        let yeet_key = YeetKey::Of(id.clone());

        let mut root_yeet: Yeet = env.storage().temporary().get(&yeet_key).expect("This fucking yeet doesnt exist");

        root_yeet.likes += 1;

        env.storage().temporary().set(&yeet_key, &root_yeet);

        env.storage().temporary().extend_ttl(&yeet_key, added_validity, added_validity);
    }

    pub fn get_yeet(env: Env, id: String) -> Yeet {
        let yeet_key = YeetKey::Of(id.clone());

        env.storage().temporary().get(&yeet_key).expect("This fucking yeet doesnt exist")
    }

    // pub fn get_yeets_by_address(env: Env, who: &Address) {}

}

mod test;