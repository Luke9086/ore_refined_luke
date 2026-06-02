use anchor_lang::pubkey;
use entropy_api::state::var_pda as entropy_var_pda;
use log::info;
use ore_api::prelude::{automation_pda, board_pda, config_pda, miner_pda};
use ore_api::state::round_pda;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_program::system_program;

/// The `ore_ev_program` (ev deploy) on-chain program.
pub const ORE_EV_PROGRAM_ID: Pubkey = pubkey!("9VFRL5mrqTYBCitfeB4gFSTRBxwi72CkwuqFvokzXdCr");
/// The real ORE program — CPI target of the ev deploy instruction.
pub const ORE_PROGRAM_ID: Pubkey = pubkey!("oreV3EG1i9BEgiAJ8b177Z2S2rMarzak4NMv1kULvWv");
/// Entropy program — invoked by the ORE deploy CPI to sample randomness.
pub const ENTROPY_PROGRAM_ID: Pubkey = pubkey!("3jSkUuYBoJzQPMEzTvkDFXCZUBksPamrVhrnHR9igu2X");

/// 1-byte instruction discriminator for `oreDeploy`.
const ORE_DEPLOY_DISCRIMINATOR: u8 = 1;

/// Build the `oreDeploy` instruction for the `ore_ev_program`.
///
/// The program reads the live ORE round, computes Kelly-optimal bets on the
/// smallest blocks, filters by the EV threshold, and deploys across them via CPI.
///
/// Instruction data is a fixed 25-byte layout:
/// `[discriminator: u8][total_amount: u64][ore_price_lamports: u64][min_ev_threshold_bps: i16][num_blocks: u8][padding: 5]`
pub fn get_ore_refined_ix(
    signer: Pubkey,
    round_id: u64,
    total_amount: u64,
    ore_price_lamports: u64,
    min_ev_threshold_bps: i16,
    num_blocks: u8,
) -> anyhow::Result<Instruction> {
    info!(
        "total_amount: {} ore_price_lamports: {} min_ev_threshold_bps: {} num_blocks: {}",
        total_amount, ore_price_lamports, min_ev_threshold_bps, num_blocks
    );

    // signer and authority are the same key in this client.
    let authority = signer;

    // Account order must match the IDL exactly.
    let accounts = vec![
        AccountMeta::new_readonly(ORE_PROGRAM_ID, false),
        AccountMeta::new(signer, true),
        AccountMeta::new(authority, true),
        AccountMeta::new(automation_pda(authority).0, false),
        AccountMeta::new(board_pda().0, false),
        AccountMeta::new(config_pda().0, false),
        AccountMeta::new(miner_pda(authority).0, false),
        AccountMeta::new(round_pda(round_id).0, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(entropy_var_pda(board_pda().0, 0).0, false),
        AccountMeta::new_readonly(ENTROPY_PROGRAM_ID, false),
    ];

    // Fixed 25-byte instruction data (little-endian).
    let mut data = Vec::with_capacity(25);
    data.push(ORE_DEPLOY_DISCRIMINATOR);
    data.extend_from_slice(&total_amount.to_le_bytes());
    data.extend_from_slice(&ore_price_lamports.to_le_bytes());
    data.extend_from_slice(&min_ev_threshold_bps.to_le_bytes());
    data.push(num_blocks);
    data.extend_from_slice(&[0u8; 5]); // padding

    Ok(Instruction {
        program_id: ORE_EV_PROGRAM_ID,
        accounts,
        data,
    })
}
