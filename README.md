# Kromer2 API

The `kromer2-api` crate provides a model for interacting with the [Kromer2](https://github.com/ReconnectedCC/kromer2) api. It omits backwards compatibility parts of the API, such as proof of work.

## Features

Both this crate, and the main Kromer2 repo are under heavy development, expect breakages in this crate as the `ReconnectedCC` devs implement more things. Please open an issue if you find any such breakages.

- **Krist API**
  - [ ] Address Endpoints
    - [x] [Get an address](https://krist.dev/docs/#api-AddressGroup-GetAddress)
    - [x] [List addresses](https://krist.dev/docs/#api-AddressGroup-GetAddresses)
    - [x] [List richest](https://krist.dev/docs/#api-AddressGroup-GetRichAddresses)
    - [x] [Get recent transactions from an address](https://krist.dev/docs/#api-AddressGroup-GetAddressTransactions)
    - [ ] [Get all names under an address](https://krist.dev/docs/#api-AddressGroup-GetAddressNames)
  - [ ] Misc. Endpoints
    - [ ] [Authenticate an address](https://krist.dev/docs/#api-MiscellaneousGroup-Login)
    - [x] [Get MOTD](https://krist.dev/docs/#api-MiscellaneousGroup-GetMOTD_+)
    - [ ] [Get latest KristWallet version](https://krist.dev/docs/#api-MiscellaneousGroup-GetWalletVersion)
    - [ ] [Get the money supply]("https://krist.dev/docs/#api-MiscellaneousGroup-GetMoneySupply")
    - [ ] [Get v2 address from private key](https://krist.dev/docs/#api-MiscellaneousGroup-MakeV2Address)
  - [ ] Name Endpoints
    - [ ] [Get a name](https://krist.dev/docs/#api-NameGroup-GetName)
    - [ ] [List names](https://krist.dev/docs/#api-NameGroup-GetNames)
    - [ ] [List newest names](https://krist.dev/docs/#api-NameGroup-GetNewNames)
    - [ ] [Get cost of a name](https://krist.dev/docs/#api-NameGroup-CheckName)
    - [ ] [Check availability of a name](https://krist.dev/docs/#api-NameGroup-CheckName)
    - [ ] [Register a name](https://krist.dev/docs/#api-NameGroup-RegisterName)
    - [ ] [Transfer a name](https://krist.dev/docs/#api-NameGroup-TransferName)
    - [ ] [Update a name](https://krist.dev/docs/#api-NameGroup-UpdateNamePOST)
  - [ ] Transaction Endpoints
    - [ ] [List all transactions](https://krist.dev/docs/#api-TransactionGroup-GetTransactions)
    - [ ] [List latest transactions](https://krist.dev/docs/#api-TransactionGroup-GetLatestTransactions)
    - [ ] [Get a transaction](https://krist.dev/docs/#api-TransactionGroup-GetTransaction)
    - [ ] [Make a transaction](https://krist.dev/docs/#api-TransactionGroup-MakeTransaction)
  - [ ] Lookup API will not be implemented until Kromer2 has made significant progress in their own implementation
  - [ ] The websocket API is planned but not until the HTTP API is complete

- **Kromr2 API**
  - [ ] Wallet API
    - [ ] Get by name
    - [ ] Get by UUID
