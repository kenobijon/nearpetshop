PET_CONTRACT="petshop.kenobi.testnet"
ACCOUNT="kenobi.testnet"
ACCOUNT2="nearatx.testnet"

# Adopt pet
near call $PET_CONTRACT adopt '{"pet_name": "Mushu"}' --account-id $ACCOUNT

# Get pet name by ID
near call $PET_CONTRACT get_pet '{"id": 0}' --account-id $ACCOUNT

# Get pet owner 
near call $PET_CONTRACT get_owner '{"id": 0}' --account-id $ACCOUNT

# Get pet by owner account id
near call $PET_CONTRACT list_owner '{"id": 0}' --account-id $ACCOUNT


# near call $PET_CONTRACT adopt '{"pet_name": "Speedy"}' --account-id $ACCOUNT
# near call $PET_CONTRACT get_pet '{"id": 1}' --account-id $ACCOUNT


# Adopt pet
near call $PET_CONTRACT adopt '{"pet_name": "Speedy"}' --account-id $ACCOUNT2

# Get pet name by ID
near call $PET_CONTRACT get_pet '{"id": 1}' --account-id $ACCOUNT2

# Get pet owner 
near call $PET_CONTRACT get_owner '{"id": 1}' --account-id $ACCOUNT2

# Get pet by owner account id
near call $PET_CONTRACT list_owner '{"id": 1}' --account-id $ACCOUNT2