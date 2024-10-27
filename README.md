# Samatha
Samantha is a WIP Steam Achievement Manager for Windows and Linux

## Build from source
### Requirements.
- Node.JS + npm
- Rust + Cargo (Check Rustup)
- Tauri requirements: https://tauri.app/v1/guides/getting-started/prerequisites/

### Compiling
1. Clone the repo
   `git clone https://github.com/LuisMayo/samantha`
2. Install dependencies
  `cd samantha; npm i`
3. If you want to just build it...
  `npx tauri build`
   Or if you prefer to run it for development purposes
   `npx tauri dev`

(You might need to clone [Steamworks-rs with this commit](https://github.com/Noxime/steamworks-rs/tree/25b12b89d528ef67dba82428d85adfad91f5f58e) manually and change the location in [src/tauri/Cargo.lock](https://github.com/LuisMayo/samantha/blob/main/src-tauri/Cargo.toml) )

## Contributing
Since this is a tiny project we don't have strict rules about contributions. Just open a Pull Request to fix any of the project issues or any improvement you have percieved on your own. Any contributions which improve or fix the project will be accepted as long as they don't deviate too much from the project objectives. If you have doubts about whether the PR would be accepted or not you can open an issue before coding to ask for my opini
on.
