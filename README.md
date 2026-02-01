# Hallownest map viewer

Simple local viewer for map from [hallownest.net](https://www.hallownest.net)

## Building

1. Download full map from https://www.hallownest.net/downloads and put in project directory
2. Change name of file in [`split.sh`](split.sh)
3. Run `./split.sh` (requires imagemagick)
	- if you use Nix, you can get it with `nix develop`
4. Run `cargo r --release -- out-webp out-webp-fixed`
5. Serve directory, for example with:
	- with python: `python -m http.server -d . 3000`
	- with [static-web-server](https://static-web-server.net): `static-web-server -d . -p 3000`
6. Open http://localhost:3000 in browser

## Images sizes

- Full image: 834 MB
- Converted to jxl: 120 MB
- Converted to webp 10x10 parts: 66 MB
- Converted to avif 10x10 parts: 43 MB

Avif is a little worse than png/jxl in terms of quality, but it's not noticeable without comparing
