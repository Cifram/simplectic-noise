# simplectic-noise
Reference implementation of simplectic noise in Rust. Note that simplectic noise is not currently recommended for use, as it has much worse performance than properly optimized Perlin noise, and produces lower quality results. It is provided here for posterity, and in case anybody finds a useful way to expand on the underlying concept.

Main implementation is in src/simplectic.rs.

Look in examples/simplectic.rs for an example of basic usage. To build and run the example, use:

    cargo build --example simplectic
