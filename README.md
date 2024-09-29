# Chatr

A chat room built using websockets and Rust (Axum), users can connect to a server through a WebSocket connection and communicate with each other in real-time.
The client application is written in svelte.

![image](https://user-images.githubusercontent.com/64641417/224826240-c6817bf4-e832-4164-b5c4-97c9725e1fec.png)
The client for the chat room is written in Svelte

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)

## Features

- [x] Message broadcasting
- [x] Websocket
- [x] Client application
- [x] Unique usernames
- [x] Join/Leave messages
- [x] Multiple Rooms
- [ ] Scrolling chat
- [ ] View members of room

## Installation

```sh
git clone https://github.com/0xLaurens/chatr
```

```sh
cd chatr
```

## Usage

### Server

Run the application

```rust
cargo run
```

### Frontend

Navigate into the frontend

```sh
cd client
```

Create a .env file in the `client` folder  containing the following variables:
```sh
PUBLIC_API_URL=http://0.0.0.0:3000
PUBLIC_WEBSOCKET_URL=ws://localhost:3000
```

Install packages using

```sh
npm i
```

Run the site

```sh
npm run dev
```
