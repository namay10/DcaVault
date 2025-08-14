# 🚀 DCA Vault 

> **Dollar Cost Averaging (DCA) Vault** - The non-custodial DCA Vault empowers freelancers, retail investors, and
small DAOs to automate recurring conversions from USDC into SOL,
significantly reducing slippage, transaction fees, and timing anxiety
compared to lump-sum swaps. By slicing purchases into scheduled
micro-swaps, the platform doesn’t just cut costs, it also cushions users against
market price uncertainty, smoothing out volatility and averaging entry prices
over the long term. With transparent, trustless smart contracts and an intuitive
UX, users retain full custody of their assets, eliminating centralized risks.
This “set-and-forget” automation simplifies SOL accumulation, enabling
disciplined dollar-cost averaging without manual intervention or complex
configurations, delivering both tangible cost savings and confidence through
every market cycle.

## ✨ Key Features
- **🔄 Automated Recurring Conversions - Automates USDC to SOL conversions on schedule without manual intervention**
- **💰 Cost Optimization - Reduces slippage and fees by slicing purchases into micro-swaps**
- **⏰ Volatility Protection - Cushions against market uncertainty through scheduled price averaging**
- **🔒 Non-Custodial Security - Users maintain full asset custody with trustless smart contracts**
- **📱 Intuitive UX - Simple setup for automated, disciplined dollar-cost averaging(Future Goal).**

## 🏗️ Architecture
### Smart Contract Structure
The project is built using the **Anchor framework** and follows a modular architecture:
```
programs/dca_vault/src/
├── lib.rs              # Main program entry point
├── constants.rs         # Program constants
├── error.rs            # Custom error definitions
├── state/              # Account state structures
│   └── vault_state.rs  # Vault state management
└── instructions/       # Program instructions
    ├── vault_init.rs   # Vault initialization
    ├── swap.rs         # Token swapping logic
    └── withdraw.rs     # Withdrawal functionality
```

### Core Components

#### 1. **Vault Initialization** (`vault_initialize`)
- Creates a new DCA vault with specified parameters
- Sets initial deposit amount, number of periods, and interval timing
- Establishes the vault owner and initial state

#### 2. **Automated Swapping** (`swap`)
- Executes periodic USDC to SOL conversions
- Integrates with Jupiter for optimal swap 
- Tracks swap completion and updates vault state

#### 3. **Withdrawal System** (`withdraw`)
- Allows vault owners to withdraw remaining Usdc before completing periods minus a fee.
- Ensures proper authorization and state validation

## 📊 Vault State Management
The vault maintains comprehensive state information:
```rust
pub struct VaultState {
    pub owner: Pubkey,
    pub total_amount: u64,
    pub periods: u16,
    pub interval_seconds: u64,
    pub staked_at: i64,
    pub curr_balance: u64,
    pub periods_completed: u16,
    pub next_swap_time: i64,
    pub total_sol_received: u64,
    pub bump: u8,
}
```

## 🧪 Testing
The program has been successfully been deployed on devnet . Transaction Signature: [5oyS79npw6ebF6tHy42CQue6T2qZtFnVrt5Y3YAZZkwf5b19uDXRV3qJgHKhhr9ab7xhimgh6cF62RCoTQNwjTM3](https://solscan.io/tx/5oyS79npw6ebF6tHy42CQue6T2qZtFnVrt5Y3YAZZkwf5b19uDXRV3qJgHKhhr9ab7xhimgh6cF62RCoTQNwjTM3?cluster=devnet)

![WhatsApp Image 2025-08-14 at 16 59 50_f8d7fac9](https://github.com/user-attachments/assets/780e4a06-3895-4099-9733-cfbc2bbcffb0)

The program has succesfully run tests against the devnet and below are ss of the same:
![WhatsApp Image 2025-08-14 at 16 34 53_c6992272](https://github.com/user-attachments/assets/ef0418c1-41be-412d-b5f3-0be3d65bceab)

![WhatsApp Image 2025-08-14 at 16 35 35_2864a949](https://github.com/user-attachments/assets/80c21499-7f46-49aa-8210-e396d2cfe6d7)

![WhatsApp Image 2025-08-14 at 16 35 46_72a50e34](https://github.com/user-attachments/assets/c610461a-adeb-4bbc-9eb2-a540d3a6ab0d)

## 🆘 Support

For questions, issues, or contributions:
- Open an issue on GitHub
- Review the test files for usage examples
- Check the Anchor documentation for framework-specific questions
  

Built this for capstone at TURBIN3 Builders Cohort.
