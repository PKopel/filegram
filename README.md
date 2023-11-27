# filegram

![tests](https://github.com/PKopel/filegram/actions/workflows/build.yml/badge.svg)

Store files as images.

## Idea

Filegram is inspired by the ["Using YouTube as Unlimited Cloud Storage??"](https://youtu.be/_w6PCHutmb4?si=TzC_hnr62YA0f8Go) video. Filegram binary allows you to encode your files as PNG images that can be stored on free image-sharing platforms. Additionally, filegram supports encrypting the file contents with ChaCha cipher.

## Packages

- [`filegram`](./filegram/): library for encodeing and decodeing files
- [`filegram-cli`](./filegram-cli/): command line interface for filegram
- [`filegram-web`](./filegram-web/): web tool build with webassembly
