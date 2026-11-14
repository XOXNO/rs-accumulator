SHELL := /bin/bash
SNIPPETS := interaction/mainnet.snippets.sh

.PHONY: add-distributor distributors

# Run once after the upgrade; signed on the Ledger (wallet index 7).
# Other address: make add-distributor BOT=erd1...
add-distributor:
	source $(SNIPPETS) && addDistributor $(BOT)

# Read back the allowlist to confirm.
distributors:
	source $(SNIPPETS) && getDistributors
