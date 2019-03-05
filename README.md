### explainshell-cli


###### explainshell.com CLI implementation in Rust
_____________

Needs the Rust Toolchain (available at [https://rustup.rs/]() ).

##### Installation:  
```cargo install --git https://github.com/p1g30n/explainshell-cli```  

>note that ~/.cargo/bin/ must be in your $PATH

##### Usage:  
```explain COMMAND [ARGS]``` (combining arguments works)

##### Example:  
``` explain tar -xzvf```

Output:

```
The GNU version of the tar archiving utility
__________________________________________________

-x, --extract, --get
      extract files from an archive
__________________________________________________

-z, --gzip, --gunzip --ungzip
__________________________________________________

-v, --verbose
      verbosely list files processed
__________________________________________________

-f, --file ARCHIVE
      use archive file or device ARCHIVE
__________________________________________________
```