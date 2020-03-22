# Image Transformer Server

---

## Dev

```
direnv allow
cargo run
```

## Example usage

```
curl -X POST  -F "data=@image.jpg" http://localhost:8088/upload  -o compressed.png
```
