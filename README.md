# build-wasm
A Command Line Interface for building web ready WebAssembly from Rust. It takes your source code, generates webassembly, and javascript glue code, ready to be loaded and used. 

## how to use
You need the rust package manager and build tool [cargo](https://www.rust-lang.org/tools/install) to make an executable. 

To use, you have to options; build executable or run from source.

### build executable
- run `cargo build --release`
- grab `./target/release/build-wasm.exe` and put it in your project folder, the one where you have `Cargo.toml`
- go to your project folder and run `build-wasm.exe` in the commandline, in *powershell*: `.\build-wasm.exe`
    - try `build-wasm.exe --help` to see options
- now run in browser

### run from source
- put this folder in your project folder
- `cargo run --manifest-path build-web/Cargo.toml --"`
    - try `run --manifest-path build-web/Cargo.toml -- --help"` to see options
    - now run in browser

You can make this command shorter by creating a cargo alias.
- go to `.cargo/config.toml` or create it
- add:

```toml
[alias]
build-wasm = "run --manifest-path build-wasm/Cargo.toml --"
```

You can now type `cargo build-wasm` instead.

## run in browser
To run your webassembly in the browser you have to load it from a *html* file. Make a `index.html` and put in the code below.

```html
<!DOCTYPE html>
<html>
    <head>
        <script type="module">
            import init from "./dist/name-of-your-project.js"
            await init()
        </script>
    </head>
</html>
```

This code loads javascript glue code which in turn loads the webassembly you wrote in rust. The final step is to host `index.html` in a *http server*. I recommend these:
- [npm http-server](https://www.npmjs.com/package/http-server)
- [python http.server](https://docs.python.org/3/library/http.server.html)
- [vscode Live Server extension](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer)
