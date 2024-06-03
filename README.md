# rust-chat :crab::left_speech_bubble:

![Rust](https://img.shields.io/badge/rust-v1.0-orange)
![License](https://img.shields.io/badge/license-MIT-blue)
![Contributions](https://img.shields.io/badge/contributions-welcome-brightgreen)

A real-time, multi-room chat application built in RUST.

This application utilizes Server-Sent Events (SSE) and JavaScript's EventSource as an alternative to WebSockets. It supports automatic reconnection with exponential backoff and live connection status.

Based on Rocket's - [chat example](https://github.com/rwf2/Rocket/blob/master/examples/chat)

## :inbox_tray: Installation

1. **Download and Install Rust:**\
Install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).

2. **Verify Installation:**\
To ensure Rust is installed correctly, run the following command in your terminal:

```bash
rustc --version
```
If the command fails, the installation was not successful.

## :hammer_and_wrench: Usage

1. **Clone the Repository:**
```bash
git clone https://github.com/yourusername/rust-chat.git
cd rust-chat
```

2. **Run the Application:**\
Use Cargo to run the application:

```bash
cargo run
```
The Application will run on [http://localhost:8000](http://localhost:8000)

## :triumph: Chuck Norris Jokes API Integration

- `/chuck help` - get help
- `/chuck` - random chuck norris Joke
- `/chuck cat` - get categories
- `/chuck @[name]` - personalized chuck norris joke
- `/chuck @[name] cat [categories]` - personalized chuck norris joke

## :bulb: Planned Upgrades

- SQLite style database for short term persistence (last 50 messages)
- Integrate more free APIs. Recommendations are welcome!
- Discord style pop-up for "/" and autocorrect

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## 📜 License

[MIT](https://choosealicense.com/licenses/mit/)
