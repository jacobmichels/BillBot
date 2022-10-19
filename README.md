# BillBot V2

A discord bot for easily sharing bills with your roommates.

## Usage

`RUST_LOG=billbot=info BILLBOT_GUILDS=<csv guild ids> cargo run`

## Features TODO

- ability to mark bill as paid
- only signal handled is SIGINT, need to handle others like SIGTERM for cleanup of slash commands
  - could use a command line flag or environment variable as a signal to BillBot to clean up slash commands then exit
- clean up the code, it's a bit of a mess

## Challenges

- docker multiplatform build takes ages on github actions, doesn't even work without swap size increase
  - need multiplatform because i'm hosting on OCI free tier arm machine
- no clear dummy-proof way of specifying who's supposed to pay for a bill, flawed options below
  - let the user use a text input field to enter the nicknames of users in the guild, this is the simplest implementation but is error prone for users (mistyping a name requires form resubmission)
    - currently using this option
  - use a fixed number of User options on the bill create slash command. this seems like it would work good as users can only specify guild members as bill payers, however the modal submit and slash command submit actions are handled in different places in the code, and it doesn't seem trivial to link the two. more investigation is needed here, but even if this did work it's still flawed because there cannot be a dynamic number of payers.
