watch-css:
	tailwindcss -i style/input.css -o style/output.css --watch

serve:
	dx serve --hot-reload=true --platform=web  

update-assets:
	cargo build --bin saturn_cli && ./target/debug/saturn_cli update-assets
