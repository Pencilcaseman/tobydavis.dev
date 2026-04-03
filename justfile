list:
    just --list --unsorted

tailwind:
    tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch

serve FLAGS="":
    dx serve {{ FLAGS }}
