# Aptos Token Launchpad

Welcome to the Aptos Token Launchpad! This project provides a framework for launching and minting tokens on the Aptos blockchain.

## Overview

The Token Launchpad allows users to create a token collection and mint event tickets. It leverages the Aptos token standard and provides essential functionalities for token management.

## Features

- *Launch Token*: Create a new token collection with specified metadata.
- *Mint Event Tickets*: Mint tickets for an event and transfer them to specified addresses.
- *Burn Tokens*: (To be implemented) Burn existing tokens.

## Prerequisites

- [Aptos CLI](https://aptos.dev/cli) installed and configured.
- A working environment with the Move language.

## Module Structure

The main module is defined as addrx::launchpad, which includes the following key components:

### Structs

- *ModuleData*: Holds the token_data_id for the launched token.
- *TokenIdData*: Contains the token_id for minted tokens.

### Constants

- *ENOT_AUTHORIZED*: Error code for unauthorized actions.

### Functions

- *launch_token*: Creates a new token collection and generates a token.
- *mint_event_ticket*: Mints tokens and transfers them to the receiver.
- *burn_tokens*: (Placeholder) Functionality for burning tokens.

## Usage

### Launch a Token

To launch a token, use the launch_token function with the following parameters:

- source_account: The account creating the token.
- collection_name: Name of the token collection.
- description: Description of the collection.
- collection_uri: URI for the collection metadata.
- token_name: Name of the token.
- token_uri: URI for the token metadata.
- maximum_supply: Maximum supply of the token.

### Mint an Event Ticket

To mint an event ticket, call the mint_event_ticket function with:

- module_owner: The account that owns the module.
- receiver: The account to receive the minted tokens.
- token_amount: The amount of tokens to mint.

### Example

A basic flow for launching a token and minting an event ticket is illustrated in the test_flow function. 

```move
@test_flow
fun test_flow(admin: signer, receiver: signer) acquires ModuleData, TokenIdData {
    // Launching token and minting tickets
}
