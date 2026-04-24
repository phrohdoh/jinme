# jinme - Clojure reader & interpreter

## License

As proprietary as possible without violating the terms, conditions, and licenses of dependencies.

I would like this project to eventually be open-source.

In other words: I have not yet consulted legal counsel on this matter to determine which licenses this can possibly be under.

## Development

### tests

```shell
cargo watch -s "$(git root)/bin/jinme test-crates --all"
```

```shell
cargo watch -s "$(git root)/bin/jinme test-crates read -- multi-line"
```

### bin files

#### `jinme`

```shell
$(git root)/bin/jinme --help
```

#### `jinme` repl

```shell
$(git root)/bin/jinme repl
```

#### `jinme` eval file

```shell
$(git root)/bin/jinme eval-file $(git root)/samples/old.jinme
```

```shell
$(git root)/bin/jinme eval-file <(printf '(prn :hi)')
```
