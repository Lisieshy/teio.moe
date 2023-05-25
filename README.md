# Teio.moe

TailwindCSS command to update styles on hot reload when developing
`npx tailwindcss -i style/input.css -o src/styles/output.css --watch`

Run the development server
`cargo leptos watch`

# creating the docker image and running it

build the docker image
`docker build . -t teiomoe`

run the docker image
`docker run -p 4200:4200 teiomoe`