# [ORE](https://github.com/regolith-labs/ore) REFINED

ORE Refined is a mining program optimized for ORE, helping you maximize the conversion of ORE to unclaimORE.

## How does it work？
The project is divided into two parts: the off-chain program and the on-chain program. This project is the off chain program and is 100% open source. The onchain program is closed source code.  the mine parameter will be passed to the onchain program, and ultimately the onchain program will calculate the EV and deploy the SOL.

## Why should I use this project to refined ORE
If you want to maximize your ORE to unclaimed ORE conversion rate, you need to deploy sol at the squares of the best EV. Calculating EV and deploying square to the optimal EV is a complex process. ORE REFINED will do it for you.

## Fee
0.5% of the deploy amount. All the fees will be used to purchase ORE or converted to unclaimed ORE. Unlocked until January 1, 2027.1

Compared with random deployment or uniform deployment to all squares, the increase in EV is far greater than the fee
![img.png](img.png)
The miner "iore" with the highest conversion rate was created by me

## How to use it

### Build from source code

1. Download the source code

```bash
$ git clone https://github.com/xintai6660707/ore_refined.git
$ cd ore_refined
```

2.  Build

```bash
$ ./cargo build --release
```


3. Create a `.env` file in the project root with your configuration:
```env
RPC_URL=https://your-rpc-endpoint
KEYPAIR_PATH=/path/to/your/keypair.json
PER_ROUND_DEPLOY_AMOUNT=0.01
ORE_REFINED_RATE=1.3
REMAINING_SLOTS=5
```

4. Run the binary
```sh
./target/release/ore-refined
```

### Pre-built binary
1. Clone the project
```bash
$ git clone https://github.com/xintai6660707/ore_refined.git
$ cd ore_refined
```
2. Create a `.env` file in the project root with your configuration:
```env
RPC_URL=https://your-rpc-endpoint
KEYPAIR_PATH=/path/to/your/keypair.json
PER_ROUND_DEPLOY_AMOUNT=0.01
ORE_REFINED_RATE=1.3
REMAINING_SLOTS=5
```

3. Run the release binary for your platform
```sh
# Linux x86_64
./target/release/ore-refined

# macOS (arm)
./target/release/ore-refined-mac

# Windows
.\target\release\ore-refined-windows.exe
```

## Mining optimization parameters

### .env variables

| Variable | Description |
|---|---|
| `RPC_URL` | Your Solana RPC endpoint URL |
| `KEYPAIR_PATH` | Path to your Solana keypair JSON file |
| `PER_ROUND_DEPLOY_AMOUNT` | SOL amount to deploy per round |
| `ORE_REFINED_RATE` | Max ORE accepted per 1 unclaimed ORE (see below) |
| `REMAINING_SLOTS` | Only deploy in the final N slots of each round (see below) |

### 1. remaining_slots
Deploy only in the final N slots of each round. For example, remaining_slots = 5 limits deployments to the last 5 slots (one slot ≈ 400 ms). Deploying later reduces the chance other deployments change the EV before your transaction lands, but setting this too low may miss rounds.
### 2. ore_refined_rate
Expected ORE required to obtain 1 unclaimed ORE. For example, ore_refined_rate = 1.3 means you accept up to 1.3 ORE per unclaimed ORE. A higher value increases deployment frequency (faster conversion) but accepts worse conversion efficiency; a lower value is stricter and results in fewer

The minimum value is 0.9, meaning that it will only be deployed when SOL worth approximately 0.9 ORE can be mined to 1 unclaimed ORE.