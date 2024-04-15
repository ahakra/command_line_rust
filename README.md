# command_line_rust

Command lines implemented
- echor
    ```
    Rust echo

    Usage: echor [OPTIONS] <TEXT>...

    Arguments:
    <TEXT>...  Input Text

    Options:
    -n, --omitnewline  Do not print newline
    -h, --help         Print help
    -V, --version      Print version
    ``` 
- catr 
    ```
    Rust cat

    Usage: catr [OPTIONS] <FILE>...

    Arguments:
    <FILE>...  Input files(s)

    Options:
    -n, --number          number all output lines
    -b, --numbernonblank  numbers non-blank lines
    -h, --help            Print help
    -V, --version         Print version
    ```

- headr
    ```
    Usage: headr [OPTIONS] [FILE]...

    Arguments:
    [FILE]...

    Options:
    -l, --lines <LINE>  [default: 10]
    -b, --bytes <BYTE>
    -h, --help          Print help
    -V, --version       Print version
    ```