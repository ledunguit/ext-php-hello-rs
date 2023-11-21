# PHP Extension in Rust

## Development
### Generate IDE stubs and build development
```shell
cargo php stubs
```

### PHP dev server with loaded extension
```shell
php -d extension=../target/debug/libkmp.dylib -S localhost:8080
```

### Version
```
1.0
```

![Image](https://github.com/ledunguit/ext-php-hello-rs/blob/master/images/extension.png?raw=true)
