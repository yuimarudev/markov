# 使い方

1. src/main.json

```json
["熱の症状", "前訴えてたから", "あかん.....", "ちょっとまずい", ...]
```

2. Build
```bash
cargo run --release
```

3. Fun
```bash
echo $(curl http://localhost:8931)
```