list:
    just --list --unsorted

tailwind:
    tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch

# [arg("platform", long="platform")]
# [arg("addr", long="addr")]
# [arg("port", long="port")]
# [arg("flags", long="flags")]
# serve profile="dev" platform="web" addr="0.0.0.0" port="8080" flags="":
#     dx serve --platform {{ platform }} --addr {{ addr }} --port {{ port }} {{ flags }}
