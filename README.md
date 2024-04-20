# ChatGPT-Cmd
Simple terminal emulator for interacting with the ChatGPT 3.5 service. Made in Rust, using the OpenAI API. It was created with the intention of using the service without the need for a web interface, focusing only on the occasional use of a simple terminal.

### Flags:

---------------

<div>
    <img src="https://img.shields.io/badge/Language%20-Rust-orange.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Operational Sys%20-Linux-yellow.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Crates%20-chat_gpt_rs, Colored-darkred.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Tools%20-OpenAI API-lightgreen.svg" style="max-width: 100%;">
    <img src="https://img.shields.io/badge/Type%20-Command line tools for utilities-beige.svg" style="max-width: 100%;">
</div>

----------------

![Usage_image](ChatGPT-cmd-use.png)

## Installation:

```bash
  git clone https://github.com//JMoreira2Dev/ChatGPT-Cmd.git
  cd ChatGPT-Cmd
  cargo build --release
```

or

```bash
  cargo install --git=https://github.com//JMoreira2Dev/ChatGPT-Cmd.git
```

## Usage: 

> ./app {OpenAI token key}

##

### Generating a ChatGPT API service key:

#### Step 1: Log in OpenAI

![Captura de tela de 2024-04-19 19-14-33](https://github.com/JMoreira2Dev/ChatGPT-Cmd/assets/167461650/30e4c3fe-e5a1-4fda-9bfe-fec25046fec1)

Select `API`

#### Step 2: Go into API keys

![Captura de tela de 2024-04-19 19-12-29](https://github.com/JMoreira2Dev/ChatGPT-Cmd/assets/167461650/ed973658-8463-4548-884b-eb0372c7f2c9)

#### Step 3: Click on the Generate secret key button and save it:

![Captura de tela de 2024-04-19 19-18-52](https://github.com/JMoreira2Dev/ChatGPT-Cmd/assets/167461650/28ad0fa9-b693-48e3-8103-d4fb68559410)

## More info (Used Crates):

- **Colored Documentation** => [Colored](https://crates.io/crates/colored)
- **Chat_gpt_rs Documentation** => [Chat_gpt_rs](https://crates.io/crates/chat-gpt-rs)
- **OpenAI API** => [API](https://www.geeksforgeeks.org/openai-python-api/)
