# Random Cut Forest C/C++

[Random Cut Forest](https://github.com/aws/random-cut-forest-by-aws) (RCF) anomaly detection for C/C++

[![Build Status](https://github.com/ankane/librcf/workflows/build/badge.svg?branch=master)](https://github.com/ankane/librcf/actions)

## Installation

Download the latest version:

- Mac - [x86_64]() or [arm64]()
- Linux - [x86_64]() or [arm64]()
- Windows - [x86_64]()

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
