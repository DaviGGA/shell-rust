A shell utility CLI application done in Rust

Reference: https://app.codecrafters.io/courses/shell

# Row to run
- Execute it in any terminal

```
cargo run
```

# Commands

## echo

```
$ echo hello world
hello world
```

##  exit (Stop the process with the status code you passed (default: 0)
```
$ exit
```

```
$ exit 1
```

## pwd  (Show your current directory path)

```
$ pwd
/home/user
```

## cd (Change your directory path)
```
$ cd ./src
$ pwd
/home/user/stc
```

## type (Check if a command is a builtin)
```
$ type echo
echo is a shell builtin
```

## exe (Execute a program)
```
$ hello_name Doe
Hello, Doe!
```



# To-do

- Better error handling
- Unit tests
- Other commands
- Interface (w/ Tauri maybe)
- Accept shell operators like & && |

