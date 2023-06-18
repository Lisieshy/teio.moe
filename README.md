# Teio.moe

Very much inspired by [tannhauser.moe](https://tannhauser.moe/). It's just that I prefer Tokai Teio. So I rewrote it in Rust because then I could combine two things I really like!

This project was made using [Leptos](https://github.com/leptos-rs/leptos) and [TailwindCSS](https://tailwindcss.com/).

The code of the gallery is copied from tannhauser.moe [repository](https://github.com/KawaiiShadowii/tannhauser.moe)'s code with permission.

# How to run website locally for development

Pre-requisites:
- [Rust Nightly](https://www.rust-lang.org/tools/install)
- [TailwindCSS](https://tailwindcss.com/docs/installation)

TailwindCSS command to update styles on hot reload when developing
```sh
$ npx tailwindcss -i style/input.css -o src/styles/output.css --watch
```

Run the development server
```sh
$ cargo leptos watch
```

Access the website at <http://localhost:4200/>

# Creating the docker image and running it

Build the docker image:

```sh
$ docker build . -t teiomoe
```

Run the docker image:
```sh
$ docker run -p 4200:4200 teiomoe
```