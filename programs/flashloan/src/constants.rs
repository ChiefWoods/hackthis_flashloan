pub const VAULT_SEED: &[u8] = b"vault";

// 0.01 SOL in lamports
pub const FLASH_LOAN_FEE: u64 = 10_000_000;

// Anchor discriminators: sha256("global:<fn_name>")[0..8]
pub const FLASH_LOAN_DISCRIMINATOR: [u8; 8] = [239, 246, 59, 224, 139, 20, 175, 14];
pub const FLASH_REPAY_DISCRIMINATOR: [u8; 8] = [182, 143, 19, 23, 39, 221, 184, 78];