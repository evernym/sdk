
import json
import os
import sys
ENTERPRISE_DID = '2hoqvcwupRTUNkXn6ArYzs'
config = {
  "agent_enterprise_verkey": "GrcPZrVRjj4Mt2G4d7QLX5VcKRz78gpgV8v3guWtvkpf",
  "enterprise_did": "2hoqvcwupRTUNkXn6ArYzs",
  "agent_pairwise_verkey": "3pCoHTVnHf9c2EPcLSrGD7xFMXDKMb8oqbbpugfW13ZT",
  "agency_pairwise_did": "YRuVCckY6vfZfX9kcQZe3u",
  "enterprise_name": "<CHANGE_ME>",
  "enterprise_verkey": "vrWGArMA3toVoZrYGSAMjR2i9KjBS66bZWyWuYJJYPf",
  "genesis_path": "<CHANGE_ME>",
  "wallet_name": "my_real_wallet",
  "agent_endpoint": "https://enym-eagency.pdev.evernym.com",
  "logo_url": "<CHANGE_ME>",
  "agent_pairwise_did": "6ASMWNRWjY6LopWeaM3QuK",
  "agency_pairwise_verkey": "J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v",
  "enterprise_did_agent": "W6FfcgDN93B6arvJEJihUw",
  "wallet_key": "walletkey"
}

FILENAME = 'utils/vcxconfig.json'


def create_config(user_config):
    for i in user_config:
        config[i] = user_config[i]
    with open(FILENAME, 'w') as out_file:
        json.dump(config, out_file, indent=4, sort_keys=True)

