# 🖼️ Image Transformer Server

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


## TODO: 
- [ ] Implement reading different args for height, width, and others
- [ ] Implement different output formats
- [ ] Implement proper errors when image is not a correct format
- [ ] Add tests
- [ ] Do deployment
