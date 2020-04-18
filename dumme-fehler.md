# Dumme Fehler

## Fehler 1

Position mit Index vertauscht, endet in seltsamen Chunk-Daten

```rust
for pos in 0..16 * 16 * 16 {
    let entry = self.data.get(pos);
    if entry as usize >= index {
        self.data.set(index, entry + 1);
    }
}
```

Erhalten: `1;2;3;1;1;1`

Erwartet: `1;3;1;1;1;1`

### Korrektur

```rust
for pos in 0..16 * 16 * 16 {
    let entry = self.data.get(pos);
    if entry as usize >= index {
        self.data.set(pos, entry + 1);
    }
}
```
