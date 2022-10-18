# BillBot V2

A discord bot for easily sharing bills with your roommates.

## Usage

`RUST_LOG=billbot=info BILLBOT_GUILDS=<csv guild ids> cargo run`

## Features TODO

- slash command for posting a bill
  - a bill posting needs to include a monetary amount, a descriptive name, and a list of people splitting the bill
  - ability to mark bill as paid
