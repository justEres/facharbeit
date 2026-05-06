# Playground

Build the browser package from the repository root:

```bash
./scripts/build-playground.sh
```

Then serve the `playground/` directory with any static file server, for example:

```bash
cd playground
python3 -m http.server 8000
```

Open `http://localhost:8000` in a browser.
