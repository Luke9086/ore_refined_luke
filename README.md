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

### Prerequisites

Before running the program, you must create a miner account on-chain:

1. Go to https://ore.supply/
2. Connect your wallet and mine once manually to create your miner account.

### Build from source code

1. Download the source code

```bash
$ git clone git@github.com:Luke9086/ore_refined_luke.git
$ cd ore_refined_luke
```

2.  Build

```bash
$ ./cargo build --release
```


3. Create a `.env` file in the project root with your configuration:
```env
RPC_URL=https://your-rpc-endpoint
KEYPAIR_PATH=/path/to/your/keypair.json
DEPLOY_BPS=400
MIN_EV_THRESHOLD_BPS=-500
REMAINING_SLOTS=15
NUM_BLOCKS=5
```

4. Run the binary
```sh
./target/release/ore-refined
```

## Mining optimization parameters

### .env variables

| Variable | Description |
|---|---|
| `RPC_URL` | Your Solana RPC endpoint URL |
| `KEYPAIR_PATH` | Path to your Solana keypair JSON file |
| `DEPLOY_BPS` | Fraction of the signer SOL balance to deploy, in basis points (100 = 1%, default: 400) |
| `MIN_EV_THRESHOLD_BPS` | Minimum acceptable expected value in basis points (see below) |
| `REMAINING_SLOTS` | Only deploy in the final N slots of each round (see below) |
| `NUM_BLOCKS` | Number of smallest blocks to target, 1-12 (see below) |

### 1. remaining_slots
Deploy only in the final N slots of each round. For example, remaining_slots = 5 limits deployments to the last 5 slots (one slot ≈ 400 ms). Deploying later reduces the chance other deployments change the EV before your transaction lands, but setting this too low may miss rounds.
### 2. min_ev_threshold_bps
Minimum acceptable expected value in basis points. For example, min_ev_threshold_bps = -500 means blocks with an EV worse than -5% are skipped. A lower (more negative) value increases deployment frequency but accepts worse expected value; a higher value is stricter and results in fewer deployments.
### 3. num_blocks
Number of smallest blocks the on-chain program targets, from 1 to 12 (default: 5). The program computes Kelly-optimal bets on the `num_blocks` smallest blocks, filters them by the EV threshold, and deploys across the ones that qualify. A smaller value concentrates the deployment on fewer blocks; a larger value spreads it wider.