# sendle

This is cli tool to send pdf files to your kindle.

In develop

## Develop

### Test
You must use nightly build to run unit tests.

Because this repository uses [CodeSandwich/Mocktopus](https://github.com/CodeSandwich/Mocktopus) as mocking libray and it depends on nightly build because of using `#![feature]`.

```
$ rustup run nightly cargo t
```
