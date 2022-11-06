# Random Cut Forest C/C++

[Random Cut Forest](https://github.com/aws/random-cut-forest-by-aws) (RCF) anomaly detection for C/C++

:evergreen_tree: Also available for [Ruby](https://github.com/ankane/random-cut-forest-ruby) and [PHP](https://github.com/ankane/random-cut-forest-php), and as a [CLI](https://github.com/ankane/rcf-cli)

[![Build Status](https://github.com/ankane/librcf/workflows/build/badge.svg?branch=master)](https://github.com/ankane/librcf/actions)

## Installation

Download the latest version:

- Linux - [x86_64](https://github.com/ankane/librcf/releases/download/v0.1.0/librcf-0.1.0-x86_64-unknown-linux-gnu.tar.gz) or [arm64](https://github.com/ankane/librcf/releases/download/v0.1.0/librcf-0.1.0-aarch64-unknown-linux-gnu.tar.gz)
- Mac - [x86_64](https://github.com/ankane/librcf/releases/download/v0.1.0/librcf-0.1.0-x86_64-apple-darwin.tar.gz) or [arm64](https://github.com/ankane/librcf/releases/download/v0.1.0/librcf-0.1.0-aarch64-apple-darwin.tar.gz)
- Windows - [x86_64](https://github.com/ankane/librcf/releases/download/v0.1.0/librcf-0.1.0-x86_64-pc-windows-msvc.zip)

You can also install it with Homebrew:

```sh
brew install ankane/brew/librcf
```

## Getting Started

Include the header

```c
#include "rcf.h"
```

Create a forest with 3 dimensions

```c
rcf_forest *forest = rcf_create(3);
```

Set parameters [unreleased]

```c
rcf_set_param(forest, "number_of_trees", "100");
```

Score a point

```c
float point[] = {1.0, 2.0, 3.0};
double score = rcf_score(forest, point);
```

Update with a point

```c
rcf_update(forest, point);
```

Free the forest

```c
rcf_free(forest);
```

## Example

```c
#include <stdio.h>
#include <stdlib.h>

#include "rcf.h"

float randf() {
    return rand() / (float) RAND_MAX;
}

int main() {
    rcf_forest *forest = rcf_create(3);
    rcf_set_param(forest, "number_of_trees", "100");

    for (int i = 0; i < 200; i++) {
        float point[] = {randf(), randf(), randf()};

        // make the second to last point an anomaly
        if (i == 198) {
            point[1] = 2;
        }

        double score = rcf_score(forest, point);
        printf("point = %d, score = %f\n", i, score);
        rcf_update(forest, point);
    }

    rcf_free(forest);
    return 0;
}
```

## Parameters

Name | Description | Default Value
--- | --- | ---
`shingle_size` | Shingle size to use | 1
`sample_size` | Points to keep in sample for each tree | 256
`number_of_trees` | Number of trees to use in the forest | 100
`random_seed` | Random seed to use | 42
`parallel` | Enable parallel execution | false

Parameter values should always be passed as strings.

```c
rcf_set_param(forest, "parallel", "true");
```

`rcf_set_params` returns zero if successful and nonzero if the name or value is invalid or if it’s called after `rcf_score` or `rcf_update`.

## Reference

- [Robust Random Cut Forest Based Anomaly Detection On Streams](https://proceedings.mlr.press/v48/guha16.pdf)

## History

View the [changelog](CHANGELOG.md)

## Contributing

Everyone is encouraged to help improve this project. Here are a few ways you can help:

- [Report bugs](https://github.com/ankane/librcf/issues)
- Fix bugs and [submit pull requests](https://github.com/ankane/librcf/pulls)
- Write, clarify, or fix documentation
- Suggest or add new features

To get started with development:

```sh
git clone https://github.com/ankane/librcf.git
cd librcf
cargo build
cargo test
```

To generate headers:

```sh
cbindgen > include/rcf.h
```
