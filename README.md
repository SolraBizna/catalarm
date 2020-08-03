This is a very simple program that reads from standard input and writes to standard output, just like `cat` run with no arguments. The twist is that, every time it reads data, it runs a command.

The command to run is given by the arguments to `catalarm`. For instance:

```sh
catalarm play -q ~/System7Sounds/9.Sosumi.flac
```

will use `sox`'s `play` command to play `9.Sosumi.flac` every time there is output.

The following (Bash-only?) command line incorporates the above:

```sh
valgrind --log-fd=3 my_buggy_program 3> \
    >(catalarm play -q ~/System7Sounds/9.Sosumi.flac 1>&2)
```

This will run `my_buggy_program` through Valgrind's memory checker, and any Valgrind output will be displayed and cause `9.Sosumi.flac` to play.

You can use it in less complicated pipelines, too. For example:

```sh
tail -F /var/log/exim4/paniclog | catalarm play -q ~/warp_core_breach.ogg
```

`tail` will watch Exim's panic log, and `catalarm` will play `warp_core_breach.ogg` any time something appears. (If the person who can hear your speakers is a Star Trek fan, this should be a very good way of getting their attention.)

Note: This is a portable program with no dependencies other than Rust itself, so it will work on Windows, even though all the above examples are intended for UNIXoids.

Note 2: `catalarm` does not wait for the command to complete before continuing. It will not run more than one command per second, but if the command takes longer than a second to run, you might still have a problem.

# License

This program is distributed under the zlib license. This puts very few restrictions on use. See [`LICENSE.md`](LICENSE.md) for the complete, very short text of the license.

