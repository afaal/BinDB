# BinPack

Is an in-binary, in-memory, sqlite database.


## Running benchmarks 
```
cargo bench 
```

## Running tests
```
cargo test
```

## Running examples 

The following will run the hello.rs example
```
cargo run --example hello
```


## Methods for finding the variable offset
Use the memory location of the variable, and subtract that from the .data segment map to find the binary offset
* Architecture dependant
* Requires file I/O

Use EGG 
* Search for the egg in the binary to find the binary offset. 
* This doesn't change between compiled binaries, so this could also be written in to the binary, so we only search for the EGG on the first run
* We can store serialized data in this blob too, which we can unserialized structs/primitives of the developers choosing
* If we maintain one address pointing to the file offset of the buffer, then we don't need the EGG after the first run.

Problems
* We cant modify the binary while it is executing 
  * Create temporary file, and move it over the previous one
  * Change the memory protected of the mmap'ed region to allow writing??
  * Create orphan processes to modify the file? 

Performance gains
* Have a temporary file along side, and just switch copy/overwrite upon commits, that way we won't have to read, and write to entirely new files. 

