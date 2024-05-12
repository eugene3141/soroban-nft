use soroban_sdk::{Env, Symbol, Address, String, Vec, BytesN};

pub(crate) fn initialized(env: &Env) {
    let topics = (Symbol::new(env, "initialized"),);
    env.events().publish(topics, true);
}

pub(crate) fn collection_updated(env: &Env) {
    let topics = (Symbol::new(env, "collection_updated"),);
    env.events().publish(topics, true);
}

pub(crate) fn freeze(env: &Env) {
    let topics = (Symbol::new(env, "freeze"),);
    env.events().publish(topics, true);
}

pub(crate) fn mint(env: &Env, owner: Address, token_id: u32) {
    let topics = (Symbol::new(env, "mint"),owner);
    env.events().publish(topics, token_id);
}

pub(crate) fn bulk_mint(env: &Env, owner: Address, tokens: Vec<(u32, String)>) {
    let topics = (Symbol::new(env, "bulk_mint"),owner);

    let mut token_ids: Vec<u32> = Vec::new(&env);

    for (token_id, _) in tokens {
        token_ids.push_back(token_id);
    }

    env.events().publish(topics, token_ids);
}

pub(crate) fn token_updated(env: &Env, token_id: u32, token_uri: String) {
    let topics = (Symbol::new(env, "token_updated"), token_id);
    env.events().publish(topics, token_uri);
}

pub(crate) fn transfer(env: &Env, token_id: u32, to: Address) {
    let topics = (Symbol::new(env, "transfer"), token_id);
    env.events().publish(topics, to);
}

pub(crate) fn approve(env: &Env, token_id: u32, spender: Address) {
    let topics = (Symbol::new(env, "approve"), token_id);
    env.events().publish(topics, spender);
}

pub(crate) fn revoke(env: &Env, token_id: u32, spender: Address) {
    let topics = (Symbol::new(env, "revoke"), token_id);
    env.events().publish(topics, spender);
}

pub(crate) fn burn(env: &Env, token_id: u32) {
    let topics = (Symbol::new(env, "burn"), token_id);
    env.events().publish(topics, true);
}

pub(crate) fn upgraded(env: &Env, hash: BytesN<32>) {
    let topics = (Symbol::new(env, "upgraded"),);
    env.events().publish(topics, hash);
}