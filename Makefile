watch-css:
	tailwindcss -i style/input.css -o style/output.css --watch

serve:
	trunk serve --port 3000

build:
	trunk build --release
