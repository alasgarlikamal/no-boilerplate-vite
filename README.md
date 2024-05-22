# No Boilerplate Vite
Too often have I found myself creating a React project and having to manually install Tailwind and remove the boilerplate that comes with Vite. So, I wrote a super-simple CLI tool to automatically scaffold a Vite project and pre-install tailwind.
### Installation
```
cargo build --release
```
Add the created `nvb` executable in `target/release/` to path or any `bin` folder to access it globally in the terminal.
### Usage
```
nbv demo-app --install
```
<img width="476" alt="Usage" src="https://github.com/alasgarlikamal/no-boilerplate-vite/assets/98516464/1fe56b24-8390-4479-8f56-91dcc5334cdc">

### Options
```
nbv --help
```
<img width="501" alt="Options" src="https://github.com/alasgarlikamal/no-boilerplate-vite/assets/98516464/0ac160b6-a3ea-4f46-861a-3c6474aa704d">

