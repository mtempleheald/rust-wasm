# Rustmen

[Micemen](https://archive.org/details/MiceMen_1020) is a game created in the 90s that I had on a 1000 games CD compendium.

Rust with WebAssembly has been on my radar for a long time but I've never completed a full project using it.
This game seems like a suitable application of the technology and one which I could learn from.

In parallel I'm doing investigations into more "enterprisey" software with rustwasm, but I need to keep these separate to reduce confusion.

Also interested in related topics:
- a fractal generator on a 2D canvas

## Status

![Continuous Integration](https://github.com/mtempleheald/rustmen/workflows/ci/badge.svg)

## Useful links

- [Rust WASM book](https://rustwasm.github.io/docs/book) - The official resource on Rust + Web Assembly
- [Doug Milford demo](https://github.com/dmilford/rust-3d-demo) - Seeing his video on YouTube motivated me to start this
- [Wasm-pack book](https://rustwasm.github.io/wasm-pack/book/) - the current/future of build of rust/wasm applications, uses npm for now
- [Wasm Bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) - wasm-bindgen reference inc WebGL examples & CI/CD
- [WebGL fundamentals](https://webglfundamentals.org/)
- [WebGL_API](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API)
- [GitHub Actions context properties](https://docs.github.com/en/actions/reference/context-and-expression-syntax-for-github-actions#github-context)
- [GitHub Actions Node example](https://stackoverflow.com/questions/58347746/automating-the-build-and-publish-process-with-github-actions-and-github-package)
- [GitHub Actions Blazor example](https://blog.zhaytam.com/2020/06/08/deploy-blazor-wasm-github-pages-using-actions/)

## Useful commands

`wasm-pack build` build wasm package
`wasm-pack test --headless --firefox` run tests in headless browser
`wasm-pack publish` publish to npm, doubt I'll be doing this.

`npm install ./www` (one-off) grab all web dependencies from package.json
`npm run start` (from www dir) run app with live reloading using webpack
`npm run build` (from www dir) build app using webpack

## Dependencies

- Rust program
  - [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) for communicating between WebAssembly and JavaScript.
  - [console_error_panic_hook](https://github.com/rustwasm/console_error_panic_hook) for logging panic messages to the developer console.
  - [wee_alloc](https://github.com/rustwasm/wee_alloc), an allocator optimized for small code size.
- Testing
  - [wasm-bindgen-test]https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/usage.html
  - [quickcheck](https://crates.io/crates/quickcheck)
- GitHub Actions 
  - [Super-Linter](https://github.com/github/super-linter)
  - [Deploy to GitHub Pages](https://github.com/marketplace/actions/deploy-to-github-pages)
  - [Node setup](https://github.com/marketplace/actions/setup-node-js-environment)
  - [wasm-pack](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/continuous-integration.html#github-actions)


### Webpack

Must support hot reload during development
- Dynamic web stuff needs to trigger on any change, this is standard practice and fast.
- WASM stuff takes longer to build, so there's a choice to make, depending whether this is a rust program running in Wasm, or a JS program which uses some Rust:
  - rebuild on any change - listen on src folder and use [Wasm Pack Webpack plug-in](https://github.com/wasm-tool/wasm-pack-plugin).
    This is the approach taken by [Doug Milford's 3D demo](https://github.com/dmilford/rust-3d-demo)  (disable auto-save)
  - manually trigger `wasm pack build`, use npm dependency on /pkg to register changes.
    This is the approach taken by [Rustwasm book](https://rustwasm.github.io/docs/book/)
    This requires 2 command windows to be open; the live reload plus anytime we want to build the wasm solution.

Must support hashing of wasm modules to facilitate cache-busting, again there are multiple approaches
- Use [HTML Webpack Plugin](https://github.com/jantimon/html-webpack-plugin) to generate hashes for us as per Doug Milford's demo.
- Magic.  I can't work out how the Rustwasm book approach does this (webpack default auto hashing?) but the output bootstrap file references _hash_.module.wasm.
  It doesn't clear out the old versions though, so I will have to add a clear step to webpack config.

Include static files from source in the output, e.g. html
- [Copy Webpack Plugin](https://www.npmjs.com/package/copy-webpack-plugin)

Want to import modules statically [if possible](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#dynamic_imports)
- Static `import * as <module> from "<module-listed-as-dependency>";`
  Loaded this way by default in rustwasm book example.
  Requires the module load to be deferred (or listen for onLoad) before execution, if triggering without user intervention.
- Dynamic `const <module-promise> = import('./pkg/<module-name>');`
  We can then trigger any commands we want to run in the then() method of the promise, e.g. single page canvas


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
- Decided to go with small WebGL game as I can probably learn more from that approach, JS boilerplate is in place, evidenced in browser console.
- Decided to get a basic game working first, something to see.  Using canvas but not WebGL yet.
- Added GitHub Actions CICD.
- Rediscovered that GitHub Pages requires index.html to live at either / or /docs.  Mine lives at /www
  Had considered changing this anyway to merge node_modules and simplify commands during local dev
- Removed the .gitignore lines for /docs and /pkg to finally see the app working in GH Pages
