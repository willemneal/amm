pub const TOKEN_DENOM: u128 = 1_000_000_000_000_000_000;

pub const MIN_FEE: u128= TOKEN_DENOM / 1_000_000;
pub const MAX_FEE: u128 = TOKEN_DENOM / 10;

pub const MIN_BOUND_TOKENS: u16 = 2;
pub const MAX_BOUND_TOKENS: u16 = 8;

pub const MAX_WEIGHT: u128 = TOKEN_DENOM * 50;
pub const MIN_WEIGHT: u128 = TOKEN_DENOM;
pub const MIN_BALANCE: u128 = TOKEN_DENOM / 1_000_000_000_000;

pub const MAX_TOTAL_WEIGHT: u128 = TOKEN_DENOM * 50;
pub const EXIT_FEE: u128 = 0;

pub const INIT_POOL_SUPPLY: u128 = TOKEN_DENOM * 100;
pub const MAX_IN_RATIO: u128 = TOKEN_DENOM / 2;

pub const MIN_POW_BASE: u128 = 1;
pub const MAX_POW_BASE: u128 = (2 * TOKEN_DENOM) - 1;
pub const POW_PRECISION: u128 = TOKEN_DENOM / 10_000_000_000;
