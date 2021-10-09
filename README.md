# kattis-rs

Just a random [kattis problem](https://open.kattis.com/problems/pachydermpeanutpacking) that I started but didn't finish

Run with the following: 

``` bash
$ cat test_case.txt | cargo run
```

Prints out the following: 

```
Boxes: [
    PeanutBox {
        x: (
            0.0,
            1.0,
        ),
        y: (
            0.0,
            1.0,
        ),
        type_: Small,
    },
    PeanutBox {
        x: (
            3.0,
            6.0,
        ),
        y: (
            2.0,
            5.0,
        ),
        type_: Large,
    },
]
Peanuts: [
    Peanut {
        x: 2.0,
        y: 0.0,
        type_: Small,
    },
    Peanut {
        x: 0.5,
        y: 0.6,
        type_: Medium,
    },
    Peanut {
        x: 3.0,
        y: 4.0,
        type_: Large,
    },
]
Boxes: [
    PeanutBox {
        x: (
            60.0,
            111.4,
        ),
        y: (
            2.9,
            32.9,
        ),
        type_: Medium,
    },
    PeanutBox {
        x: (
            20.0,
            43.4,
        ),
        y: (
            82.6,
            153.1,
        ),
        type_: Small,
    },
    PeanutBox {
        x: (
            34.7,
            56.6,
        ),
        y: (
            9.6,
            78.2,
        ),
        type_: Large,
    },
]
Peanuts: [
    Peanut {
        x: 60.0,
        y: 8.3,
        type_: Small,
    },
    Peanut {
        x: 43.4,
        y: 153.1,
        type_: Small,
    },
    Peanut {
        x: 13.4,
        y: 55.9,
        type_: Medium,
    },
    Peanut {
        x: 61.5,
        y: 68.1,
        type_: Medium,
    },
    Peanut {
        x: 72.1,
        y: 69.1,
        type_: Large,
    },
    Peanut {
        x: 78.4,
        y: 13.2,
        type_: Large,
    },
]
```
