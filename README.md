# Chatr
A chat room built using websockets and Rust (Axum), users can connect to a server through a WebSocket connection and communicate with each other in real-time. 
The client application is written in svelte.

![image](https://user-images.githubusercontent.com/64641417/224757083-dcb7218e-614a-489e-a823-0ceefef23bcd.png)
The client for the chat room is written in Svelte

## Table of Contents
* [Features](#features)
* [Installation](#installation)
* [Usage](#usage)

## Features
- [x] Message broadcasting
- [x] Websocket
- [x] Client application
- [ ] Unique usernames
- [ ] Multiple Rooms
- [ ] Join/Leave messages

## Installation
```sh
git clone https://github.com/0xLaurens/chatr
```
```sh
cd chatr
```

## Usage

### Server
Install the required dependancies
```rust
cargo install
```
Run the application
```rust
cargo run
```
### Frontend
Install packages using
```sh
npm i 
```
Run the site
```sh
npm run dev
```
