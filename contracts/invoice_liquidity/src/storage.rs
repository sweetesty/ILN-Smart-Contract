use soroban_sdk::{contracttype, Address, Env};

use crate::config::Config;
use crate::invoice::{AppealRecord, Invoice, LpFundRequest, ReputationScore};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataKey {
    // Instance Storage
    Admin,
    Config,
    FeeRate,
    MaxDiscountRate,
    DistributionContract,
    Paused,
    /// Minimum payer reputation required to fund an invoice (Issue #28). Default 0.
    MinPayerReputation,
    NextInvoiceId,

    // Persistent Storage
    Invoice(u64),
    InvoiceCount,
    Token,
    PayerScore(Address),
    InvoiceFunders(u64),
    ApprovedToken(Address),
    TokenList,
    /// Detailed reputation profile per address (Issue #26).
    Reputation(Address),
    Appeal(u64),
    PreDefaultPayerScore(u64),
    LpScore(Address),
    FundQueue(u64),
    QueueResolution(u64),

    // Stats (Persistent)
    TotalInvoices,
    TotalFunded,
    TotalPaid,
    TotalVolumeUsdc,
    TotalVolumeEurc,
    TotalVolumeXlm,
    TokenVolume(Address),
    Dispute(u64),
    SubmitterInvoices(Address),
    LpInvoices(Address),
    /// Fixed-size min-heap of the top payers by reputation score (Issue #77).
    TopPayersHeap,
}

// ----------------------------------------------------------------
// Config Helpers
// ----------------------------------------------------------------

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_config(env: &Env) -> Option<Config> {
    env.storage().instance().get(&DataKey::Config)
}

pub fn set_config(env: &Env, config: &Config) {
    env.storage().instance().set(&DataKey::Config, config);
}

pub use crate::invoice::{
    save_invoice, load_invoice, invoice_exists, read_next_invoice_id, write_next_invoice_id,
    next_invoice_id, get_payer_score, set_payer_score, get_lp_score, set_lp_score,
    get_invoice_funders, save_invoice_funders, get_appeal, save_appeal,
    save_pre_default_payer_score, get_pre_default_payer_score, get_fund_queue, save_fund_queue,
    get_queue_resolution, save_queue_resolution, get_contract_stats, add_volume,
    increment_total_invoices, increment_total_funded, increment_total_paid, is_paused, set_paused,
};
