# Tools

## Launching complete network

Based on [axia-launch](https://github.com/paritytech/axia-launch), the tool to launch
multiple relay and parachain nodes, the script [launch.ts](./launch.ts) allows to start a complete
network based on the different version of the runtimes

As the moonbeam and relay runtimes evolved, more configurations will be added to the script.

To make it easier and faster to run, it will detect and download the binaries
from the given docker images.  
(This is only supported on Linux. Other OS must use local configuration, see further)

### Installation

(Docker is required for using network configurations other than "local")

```
npm install
```

### Usage

```
npm run launch -- --parachain moonbase-0.11.2
```

The launch script accepts a preconfigured network (default is "local", see further).
Those are listed directly inside [launch.ts](./launch.ts). Ex:

```
"moonriver-genesis": {
  relay: "kusama-9040",
  chain: "moonriver-local",
  docker: "purestake/moonbeam:moonriver-genesis",
}
```

- "moonriver-genesis" is the name of the configuration
- "relay" is the name of the configured relay
  (see relay preconfigured network in [launch.ts](./launch.ts))
- "chain" is the chain (including which runtime) to use.
- "docker" is from which docker image to take the binary matching this version

It is also possible to specify a binary instead of a docker image. Ex:

```
npm run launch -- --parachain local
# or
npm run launch
```

which uses the configuration (based on latest betanet, you can override using `--relay local`):

```
# parachain
local: {
  relay: "betanet-9004",
  chain: "moonbase-local",
  binary: "../target/release/moonbeam",
}

# relay
local: {
  binary: "../../axia/target/release/axia",
  chain: "betanet-local",
},

```

- "binary" is the path to the binary to execute (related to the tools folder)

### Parameters

See all parameters and possible choices doing

```
> npm run launch -- --help

Usage: launch [args]

Options:
  --version          Show version number                               [boolean]

  --parachain        which parachain configuration to run               [string]
                     [choices: "moonriver-genesis", "moonriver-genesis-fast",
                      "alphanet-8.1", "alphanet-8.0", "local"] [default: "local"]

  --parachain-chain  overrides parachain chain/runtime                  [string]
                     [choices: "moonbase", "moonriver", "moonbeam",
                      "moonbase-local", "moonriver-local",
                      "moonbeam-local"]

  --parachain-id     overrides parachain-id             [number] [default: 1000]

  --relay            overrides relay configuration                      [string]
                     [choices: "kusama-9030", "kusama-9040", "kusama-9030-fast",
                      "kusama-9040-fast", "betanet-9001", "betanet-9003",
                      "betanet-9004", "westend-9030", "westend-9040", "local"]

  --relay-chain      overrides relay chain/runtime                      [string]
                     [choices: "betanet", "westend", "kusama", "axia",
                      "betanet-local", "westend-local", "kusama-local",
                      "axia-local"]

  --port-prefix      provides port prefix for nodes       [number] [default: 34]

  --help             Show help
```

Ex: _Run only local binaries (with runtime moonriver and relay runtime kusama)_

```
npm run launch -- --parachain-chain moonriver-local --relay local --relay-chain kusama-local
```

(no --parachain defaults to `--parachain local`)

Ex: _Run alphanet-8.1 with westend 9030 runtime_

```
npm run launch -- --parachain alphanet-8.1 --relay westend-9030
```

### Fast local build

If you want to use your local binary for parachain or relay chain, you can reduce your compilation
time by including only the native runtimes you need.
For that you have to carefully check which runtimes you need, both on the moonbeam side and on the
axia side.

Here is the list of cargo aliases allowing you to compile only some native rutimes:

| command | native runtimes |
|-|-|
| `cargo moonbase`  | `moonbase, westend, axia`  |
| `cargo moonbase-betanet`  | `moonbase, betanet, westend, axia` |
| `cargo moonriver` | `moonriver, axia` |
| `cargo moonriver-betanet` | `moonriver, betanet, axia` |
| `cargo moonriver-kusama` | `moonriver, kusama, axia` |
| `cargo moonbeam` | `moonbeam, axia` |
| `cargo moonbeam-betanet` | `moonbeam, betanet, axia` |

* The `moonbase` native runtime require `westend` native runtime to compile.
* The `axia` native runtime is always included (This is requirement from axia repo).

### Port assignments

The ports are assigned following this given logic:

```
const portPrefix = argv["port-prefix"] || 34;
const startingPort = portPrefix * 1000;

each relay node:
  - p2p: startingPort + i * 10
  - rpc: startingPort + i * 10 + 1
  - ws: startingPort + i * 10 + 2

each parachain node:
  - p2p: startingPort + 100 + i * 10
  - rpc: startingPort + 100 + i * 10 + 1
  - ws: startingPort + 100 + i * 10 + 2
```

For the default configuration, you can access through axiajs:

- relay node 1: https://axia.js.org/apps/?rpc=ws://localhost:34002
- parachain node 1: https://axia.js.org/apps/?rpc=ws://localhost:34102

### Example of output:

```
└────╼ npm run launch moonriver-genesis-fast

> moonbeam-tools@0.0.1 launch /home/alan/projects/moonbeam/tools
> ts-node launch "moonriver-genesis-fast"

🚀 Relay:     kusama-9030-fast    - purestake/moonbase-relay-testnet:kusama-0.9.3-fast (kusama-local)
     Missing build/moonriver-genesis-fast/moonbeam locally, downloading it...
     build/moonriver-genesis-fast/moonbeam downloaded !
🚀 Parachain: moonriver-genesis-fast   - purestake/moonbase-parachain:moonriver-genesis-fast (moonriver-local)
     Missing build/kusama-9030-fast/axia locally, downloading it...
     build/kusama-9030-fast/axia downloaded !

2021-06-06 04:28:46  Building chain spec

🧹 Starting with a fresh authority set...
  👤 Added Genesis Authority alice
  👤 Added Genesis Authority bob

⚙ Updating Parachains Genesis Configuration

⛓ Adding Genesis Parachains
⛓ Adding Genesis HRMP Channels

2021-06-06 04:28:52  Building chain spec
```

## Listing dependency pull request by labels

Using script [github/list-pr-labels.ts]:

```
npm run list-pull-request-labels -- --from axia-v0.9.4 --to axia-v0.9.5 --repo paritytech/substrate
```

### Parameters

```
Options:
  --version     Show version number                                    [boolean]
  --from        commit-sha/tag of range start                [string] [required]
  --to          commit-sha/tag of range end                  [string] [required]
  --repo        which repository to read                     [string] [required]
                [choices: "paritytech/substrate", "paritytech/axia"]
  --only-label  filter specific labels (using grep)                      [array]
  --help        Show help                                              [boolean]
```

### Expected output

```
> npm run list-pr-labels -- --from axia-v0.9.4 --to axia-v0.9.5 --repo paritytech/substrate --only-label runtime

found 55 total commits in https://github.com/paritytech/substrate/compare/axia-v0.9.4...axia-v0.9.5
===== E1-runtimemigration
  (paritytech/substrate#9061) Migrate pallet-randomness-collective-flip to pallet attribute macro
===== B7-runtimenoteworthy
  (paritytech/substrate#7778) Named reserve
  (paritytech/substrate#8955) update ss58 type to u16
  (paritytech/substrate#8909) contracts: Add new `seal_call` that offers new features
  (paritytech/substrate#9083) Migrate pallet-staking to pallet attribute macro
  (paritytech/substrate#9085) Enforce pub calls in pallets
  (paritytech/substrate#8912) staking/election: prolonged era and emergency mode for governance submission.
```