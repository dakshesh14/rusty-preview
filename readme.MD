# Rushy preview

A rust microservice to fetch metadata of a given URL.

## Installation

1. Clone the repository
2. Run cargo migrate
3. Run cargo run or cargo run server to start the server

## Usage

1. Make a GET request to /preview?url=<url> to get the metadata of the given URL.

## Features

1. Fetch metadata of a given URL using OpenGraph.
2. Uses headless browser to fetch metadata for SPA websites.
3. Blazing fast.
4. Dockerized (Only for development environment)

## Future Scope

- [x] Fully dockerize development environment
- [x] Add redis caching
- [ ] Fully dockerize production environment
- [ ] Save images in S3 and serve them from there
- [ ] API authorization?

## Contributing

All contributions are welcome. Please create an issue before making a PR for big changes.

## Acknowledgements

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [Tokio](https://tokio.rs/)

and many more awesome people who have contributed to the Rust ecosystem.

## Authors

- [Dakshesh Jain](https://dakshesh.me)

## License

This project is licensed under the GPL-3.0 License - see the [LICENSE](LICENSE) file for details.
