# Shortest path in Rust

The goal of this first project is to write a program that can compute the
shortest path between two nodes in a weighted graph. The input will be provided
as a single file, edges.txt.

The file will have the following format:

```txt
1 2 10
2 3 20
1 3 5
```

We will use the following conventions:

- Everythin after a line that contains "---" should be ignored
- Node names are integers and are implicitly defined by edges, if no edges, no
  node.
- Each row has the format `<src> <dst> <weight>`
- If any row is invalid, the program should reject the file

### Running the script

The script should expose an executable named `shortest` that takes 3 positional
arguments: `<filepath> <src> <dest>` and should fail if any if the commands is
not valid
