# Rallo - demo

This repo contains a demo of using Rallo allocator with:
- Strings
- Hashbrown
- IndexMap
- Regex
- SQLx with Postgres (+ Tokio)

It tracks memory allocations and generates flamegraphs.

For run the SQLx demo, ensure you have a Postgres instance running. You can use Docker to quickly set one up:

```
docker run --name some-postgres \
    -e POSTGRES_PASSWORD=mysecretpassword \
    -p5432:5432 -d \
    postgres:17
```
