# Install PGRX
```bash
cargo install --locked cargo-pgrx
````

```bash
cargo pgrx init
```

To then create a new extension :
```bash
cargo pgrx new my_extension
cd my_extension
```

To create an installation package directory: 
```bash
cargo pgrx package
```

This will by default as output in the `./target/[debug|release]/extname-pgXX/` directory.

To install the extension in an arbitrary directory:
```bash
cargo pgrx package --out-dir my_own_extension
```


# Building in a Docker container
```bash
docker build --tag 'pgrx_validation' ./Dockerfile
```

Now it should be available in the list of images :
```bash
docker images ls
```

And then to attach to the container:
```bash
docker run -it aa245c62a1c9 (This is my ID)
```
