# Flush shell

A command line shell written in Rust.

```
~/Downloads〉catsay meow!
               _----------_
              /            \
             |    meow!    |
    _---_    |  _________.-
  ／＞　 フ    \/
  | 　_　_|
／` ミ＿xノ
／　　　　 |
(　 ヽ＿ヽ_)__)
＼二つ
```

# Installition

To be able to call all of the available internal commands, you'll first need to run build in the root directory.

```
cargo build
```

After this, you can launch flush directly from ./target/debug/flush or by doing:

```
cargo run --bin flush
```

# Usage

You can list all external and internal commands by typing 'help'.

The source code for all built-in commands is located in the 'commands' subdirectory (with the exception of ls, which has to be integrated into the shell directly).
