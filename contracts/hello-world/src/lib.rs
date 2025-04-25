#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Symbol, symbol_short};

#[contract]
pub struct ProjectDetailsContract;

#[contractimpl]
impl ProjectDetailsContract {
    const PROJECT_TITLE: Symbol = symbol_short!("PROJ_TTL");
    const PROJECT_DESC: Symbol = symbol_short!("PROJ_DESC");

    pub fn set_project_details(env: Env, title: String, description: String) {
        env.storage().instance().set(&Self::PROJECT_TITLE, &title);
        env.storage().instance().set(&Self::PROJECT_DESC, &description);
        // log!(&env, "Project details updated - Title: {}, Description: {}", title, description);
    }

    pub fn get_project_title(env: Env) -> String {
        env.storage().instance().get(&Self::PROJECT_TITLE).unwrap_or(String::from_str(&env, ""))
    }

    pub fn get_project_description(env: Env) -> String {
        env.storage().instance().get(&Self::PROJECT_DESC).unwrap_or(String::from_str(&env, ""))
    }
}