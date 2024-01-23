# pluralistic-rs

## Overview

The `matcha_funding` library is a Rust crate providing functionality to calculate quadratic funding distributions based
on a set of contributions. The library includes a robust implementation for generating random contributions, managing
contributions, and applying quadratic funding formulas to distribute a matching pot of funds.

## Features

- **Quadratic Funding Calculation**: Calculate funding distributions using the `calculate_linear_qf` function.
- **Matching Upscale**: Upscale matching to saturate a round.
- **Matching Cap Strategies**: Apply different cap strategies (`Cap` and `Redistribute`) using `MatchingCapStrategy`.
- **Random Contribution Generation**: Generate random contributions with the `Random` trait implemented
  for `Contribution`.

## Installation

Add `matcha_funding` to your `Cargo.toml` dependencies:

```toml
[dependencies]
matcha_funding = "0.1.0"
```

## Usage

1. **Defining Contributions**: Contributions are represented by the `Contribution` struct, which includes sender,
   recipient, and amount details.

1. **Calculating Distributions**: Use `calculate_linear_qf` to calculate the quadratic funding distribution based on the
   contributions, matching pot, and options provided.

2. **Applying Cap Strategies**: Utilize the `MatchingCapStrategy` to define how contributions are capped or
   redistributed when necessary.
3. **Generating Random Contributions**: Use the `Random` trait to generate random contributions for testing and
   simulation purposes.

## Example

```rust
use matcha_funding::{Contribution, calculate_linear_qf, LinearQfOptions, Random};

fn main() {
    // Generate random contributions
    let contributions = vec![Contribution::rnd(), Contribution::rnd(), ...];

    // Define options for quadratic funding calculation
    let options = LinearQfOptions {
        matching_cap_amount: Some(1000.0),
        matching_cap_strategy: MatchingCapStrategy::Cap,
        ..Default::default()
    };

    // Calculate distributions
    let distributions = calculate_linear_qf(contributions, 5000.0, options);

    // Process distributions...
}
```

## Testing

The library includes a `tests` module with test cases to ensure the functionality works as expected.

## Dependencies

- `rand`: Used for generating random contributions and other randomness-related functionalities.

## License

MIT