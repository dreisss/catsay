[**cowsay**]: https
[**rust**]: https

# [**CATSAY**](#catsay)

```
         _-------------------_
        |  Hello, I'm a cat!  |
       / ¯-------------------¯
      /
 ／l、
（ﾟ､ ｡ ７
|、ﾞ ~ヽ
じし f\_, )ノ
```

My simple version of the [**cowsay**] program, but with a cat. You can run using the command
line pipe or the default input, just like this:

```sh
fortune | catsay # With pipe
catsay "Hello, I'm a cat!" # default cli input
```

## [**Installation**](#running-locally)


It was made with [**rust**], so just follow the default crate installation. Run the commands
below:

```sh
git clone https://github.com/dreisss/catsay.git
cd catsay
cargo install
```

And to verify if everything is ok, run:

```sh
catsay -v
```

## [**Commands**](#commands)

Useful built-in commands in the program:

- [**help**](#)

  Help command to the program. Using:

  ```sh
  catsay -h # or catsay --help
  ```

- [**version**](#)

  Display the program version. Using:

  ```sh
  catsay -v # or catsay --version
  ```
