use soroban_sdk::{contracttype, contracterror, Address, String, Map, Env, panic_with_error};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const PERSISTENT_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const PERSISTENT_LIFETIME_THRESHOLD: u32 = PERSISTENT_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone, Debug)]
#[contracttype]
pub enum DataKey {
    MinterFrozen,
    Frozen,
    CollectionInfo,
    Token(u32),
    TokensCount,
    MaxTokenId,
    Operator(Address, Address)
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct CollectionInfo {
    pub admin: Address,
    pub creator: Address,
    pub minter: Address,
    pub name: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub external_link: Option<String>,
    pub explicit_content: bool,
    pub start_trading_time: Option<u64>,
    pub royalty_info: Option<RoyaltyInfo>,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct RoyaltyInfo {
    pub payment_address: Address,
    pub share: u32,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct TokenInfo {
    pub owner: Address,
    pub approvals: Map<Address, Expiration>,
    pub token_uri: String,
}

impl TokenInfo {
    pub fn check_can_approve(&self, env: &Env, from: Address) {
        if self.owner == from {
            return; 
        }

        let op: Option<Expiration> = env.storage().persistent().get(&DataKey::Operator(self.owner.clone(), from)).unwrap();
        match op {
            Some(ex) => {
                if ex.is_expired(&env) {
                    panic_with_error!(&env, Error::NotAuthorized)
                }
            }
            None => panic_with_error!(&env, Error::NotAuthorized),
        }
    }
    pub fn check_can_send(&self, env: &Env, from: Address) {
        if self.owner == from {
            return; 
        }

        if self.approvals.contains_key(from.clone()) && !self.approvals.get(from.clone()).unwrap().is_expired(&env) {
            return; 
        }

        if env.storage().temporary().has(&DataKey::Operator(self.owner.clone(), from)) {
            return;
        }

        panic_with_error!(&env, Error::NotAuthorized);
    }
}

#[derive(Copy, Clone, Debug)]
#[contracttype]
pub enum Expiration {
    AtLedger(u32),
    AtTime(u64),
    Never
}

impl Default for Expiration {
    fn default() -> Self {
        Expiration::Never {}
    }
}

impl Expiration {
    pub fn is_expired(&self, env: &Env) -> bool {
        let ledger = env.ledger();
        match self {
            Expiration::AtLedger(sequence) => ledger.sequence() >= *sequence,
            Expiration::AtTime(time) => ledger.timestamp() >= *time,
            Expiration::Never {} => false,
        }
    }
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 0,
    Initialized = 1,
    Frozen = 2,
    MinterFrozen = 3,
    InvalidCollectionInfo = 4,
    NotNFT = 5,
    AlreadyMinted = 6,
    NotAuthorized = 7
}