# prometrics_exporter

A simple exporter for [prometrics](https://docs.rs/prometrics/) to create an
endpoint for Prometheus to scrape.

## Example
```rust
fn main() {
    prometrics_exporter::start("127.0.0.1:9091").unwrap();

    my_mod::do_thing();
}

mod my_mod {
    use prometrics::metrics::CounterBuilder;

    pub fn do_thing() {
        let counter = CounterBuilder::new("tasks").finish().unwrap();

        loop {
            let result = work();
            counter.add_u64(result.completed);
        }
    }
}
```

This will listen on http://localhost:9091/metrics and serve all metrics available
to the default getherer.
