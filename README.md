# Readme

A project for me to get to grips with Rust and Web Assembly.
I dabbled a while back, before wasm-pack but time got away from me.
Not sure what this will end up being, likely one of:
- a small, probably 2D game
- seeing what a boring enterprise web app could be if made more "gamey"

I know it isn't the official WASM line but I really am interested in a world without JavaScript, or any runtime errors.
Svelte and Elm also interest me, but could wasm be the silver bullet?

## Useful links

- [Rust WASM book](https://rustwasm.github.io/docs/book/introduction.html) - The official resource on Rust + Web Assembly
- [Doug Milford demo](https://github.com/dmilford/rust-3d-demo) - Seeing his video on YouTube motivated me to start this
- [Wasm-pack book](https://rustwasm.github.io/wasm-pack/book/) - the current/future of build of rust/wasm applications, uses npm for now

## Useful commands

`wasm-pack build` build wasm package
`wasm-pack test --headless --firefox` run tests in headless browser 
`wasm-pack publish` publish to npm, doubt I'll be doing this.

`npm install ./www` (one-off) grab all web dependencies from package.json 
`npm run start` (from www dir) run app with live reloading using webpack
`npm run build` (from www dir) build app using webpack


## Rust dependencies

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook) for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized for small code size.


### Webpack

Must support hot reload during development
- Dynamic web stuff needs to trigger on any change, this is standard practice and fast.
- WASM stuff takes longer to build, so there's a choice to make:
  - rebuild on any change - listen on src folder and use [Wasm Pack Webpack plug-in](https://github.com/wasm-tool/wasm-pack-plugin).
    This is the approach taken by [Doug Milford's 3D demo](https://github.com/dmilford/rust-3d-demo)  (disable auto-save)
  - manually trigger `wasm pack build`, use npm dependency on /pkg to register changes.
    This is the approach taken by [Rustwasm book](https://rustwasm.github.io/docs/book/) with little discussion

Must support hashing of wasm modules to facilitate cache-busting, again there are multiple approaches
- Use [HTML Webpack Plugin](https://github.com/jantimon/html-webpack-plugin) to generate hashes for us as per Doug Milford's demo.
- Magic.  I can't work out how the Rustwasm book approach does this (webpack default auto hashing?) but the output bootstrap file references <hash>.module.wasm.
  It doesn't clear out the old versions though, so I will have to add a clear step to webpack config.

Include static files from source in the output
- [Copy Webpack Plugin](https://www.npmjs.com/package/copy-webpack-plugin) 


## Change History

I have no intention of keeping this section up to date long term, however I always regret not making notes as I go along, trying to learn from my mistakes.

- Set up empty project in GitHub, Rust .gitignore as this is the bulk of the code
- configure environment and add missing dependencies, following the [rustwasm book](https://rustwasm.github.io/docs/book/game-of-life/setup.html)
- `cargo generate --git https://github.com/rustwasm/wasm-pack-template.git` - initialise the wasm part of the solution
  They expected this to be a new folder, I started in GitHub, I have to manually merge and fix project name
- remove .travis.yml and .appveyor.yml - I intend to use GitHub Actions for my build, [clearly](https://blog.rust-lang.org/inside-rust/2020/07/23/rust-ci-is-moving-to-github-actions.html) it is an appropriate choice.
- `npm init wasm-app www` - initialise the web part of the solution
  Includes webpack & webpack-dev-server but needs a lot of personalisation, almost easier to start from scratch?
- `npm install ./www` install web dependencies (one-off)
- tidy up package.json, not likely to publish but kept some identity stuff anyway, removed unneccesary hello-wasm-pack dependency.
- Discover that VS Code now hides .git folder by default `File > Settings > files.exclude`, remove hidden git submodule
- 



