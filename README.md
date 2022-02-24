# RTIC-zero app playground

## Design

`rtic_zero` intend to follow expected file structure for Cargo projects.

File structure:

- `src/lib.rs` library for the the application
  Must include:
  ```rust
  // Machine generated, DO NOT TOUCH!
  pub mod rtic_zero;
  ```
  Can (optionally) contain application dependent library items.

- `src/rtic_zero.rs` generated code for 'rtic`
  
- Multiple targets supported:
  - `main.rs` 
  - `bin/app_variant/main.rs`

- `examples/ex1.rs` examples supported
  
## User Code

`rtic_zero` intend to follow the task/resource structure used by `cortex-m-rtic` v1.0.0.

The main difference is that the Task/Resource model is separated from the user code. Based on the Task/Resource model code is generated in `src/rtic_zero.rs` and can be seen as part of the application. (This can of course be said for `cortex-m-rtic` as well, but in that case code generation/expansion is done "in place").

## Open Questions

For now this is just a mockup/playground for experimentation. There are various alternatives to:

- RTIC-Zero model representation:
  - toml
  - xml
  - json
  - Rust
  - Custom DSL

  Toml/xml/json has the benefits of mature serde support. Rust/Custom DSLs might have better readability, hand to tell without further experimentation.

  Based on serde it should be quite easy to come up with working examples for evaluation.
  
- Intermediate code-generation and its representation:
  
  One of the design goals it to offer modularity and multi-pass compilation.
  Potentially allowing chains like:

  Timed Scheduling -> Message Passing -> Deadlines to Fixed Priorities -> Performance Logging ->  RTIC-Zero -> ThumbV7m

  Where each pass of compilation can be opted in/out dependent on the application needs.

  Passes could be implemented in separate crates and distributed under different licenses. E.g., asserting requirements for certification is probably needed only for safety critical commercial applications.

  Passes could be replaced by alternative implementations, optimized for given requirement (e.g., trading memory efficiency to computational complexity).

  In order for a modular framework to operate, each pass must generate a valid model for the successive steps, transitively leading up to a valid `rtic-zero` model.

  We need to come up with a reasonable error reporting mechanism that makes it clear what went wrong, why and haw the user should address the problem.

  Some errors might be spotted as being an illegal model for the pass at hand, some errors later (ultimately at compile time if the user code is at fault), some even later (at link time).

  How error detection, propagation and reporting should be be solved requires further experimentation. 

- Final pass(es):

  `rtic-zero` is the "final destination" in terms of the SRP based Task/Resource model. From there, interrupt bindings, lock implementations etc. are architecture dependent.

  In this setting it easy to foresee architecture specifics to be treated by separate crates.

  This also opens up for multi-core implementations mapping multiple Task/Resource sets to individual cores communicating between each other over shared memory under control of the RTIC-Zero code generation.

## Next step(s)

Concrete next steps are to experiment with a Task/Resource model representation for the `rtic-zero` pass, glue code generation and backend bindings for the `cortex-m` architecture.
 

