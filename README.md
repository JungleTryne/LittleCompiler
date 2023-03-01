# LittleCompiler

My little compiler of custom assembly for the toy [virtual machine](https://github.com/JungleTryne/VMachine).

## Warning

The syntax is clumsy (for example, there must be a new line at the end of the source), but eventually
I will try to fix it. Also, compiler errors are not that informative, so make sure that you are
writing the right instructions.

## Instructions

To learn about architecture and available instructions, please refer to the [docs of the
virtual machine](https://github.com/JungleTryne/VMachine/blob/master/docs/instructions.md)

## Examples

To see the examples, check `examples` folder.

## How to run?

```bash
cargo run <path_to_source> <path_to_compiled_bin>
```

For example:

```bash
cargo run ./examples/fibonacci.las ./bin/fibonacci.bin
```