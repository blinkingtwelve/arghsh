# ARGHSH

## What is this

A login shell for SSH RPC that safely passes through your argument vector.
For Posix-y systems that have `execv`.

## Why is this

When we use SSH as an RPC transport mechanism, like so (trivialised case, Python):

```python
from subprocess import run
run(['ssh', 'somewhere', 'ls' '-la', 'a filename with spaces'])
```

then sooner or later we will find out that the OpenSSH server on the remote side doesn't execute `ls` directly — rather, it starts the login shell (as referenced from `/etc/passwd` or from some other source) with two arguments: 

1. `-c`
2. `ls -la a filename with spaces`.

It does that even when you use `ForceCommand`.
In other words, the argument vector framing we so neatly passed to `ssh` was lost; it's been string-joined and passed on as a single argument to the login shell `>:(`.

The invoked login shell (say, `bash`) will then proceed to tokenize its second argument, in the same way as it does when you use the shell interactively. Sadly for us, it will not simply reconstruct the original three-member argument vector of `['ls', '-la', 'a filename with spaces']`. It turns out that we'll need to escape the spaces in `a filename with spaces`. Or we could quote the filename part of the command. That would indeed suffice for this trivial case, but to handle arbitrary inputs we'll also need to escape any quoting and escaping characters already present in those inputs. And take care of all the shell metacharacters that might occur. Going down this rabbit hole is not necessarily pretty, pleasant, or easy. It may even be ű͚̥̼̩̳̭̫́ͯ̍͢n̓͗̈́̈s̘͈̠̠̲̾͐͊ͦa̡̱̯̖̋́̋ͥͣḟ̸̹͎͇̠̫̃ͫ̑ͦḛ̯͍̰ͣ͋̋͑ͅ, exposing you to injection if parts of the argument vector are unsafe inputs. The misery grows with every round of tokenization (as when your RPCd command does RPC in turn — as in `ssh somewhere ssh somewhere_else ls -la a filename with spaces`). **ARGH!**

So. It would have been much better if the original argument vector element boundaries had been preserved, using well-understood and simple framing and escaping. A JSON array of strings is a vehicle approachable from many programming languages, so we'll use that, and we will simply avoid having any shells in the loop. In such a setup, `argsh` takes the role of login shell.

## Usage

Build `arghsh`. For creating a statically linked executable, read onwards.
Then use `chsh` to set `arghsh` as a login shell. You'll probably want a dedicated user for SSH RPC through `arghsh`, 
as this shell is a bit \*cough\* awkward to use as a day-to-day login shell.
If for some reason you don't want a completely distinct user, then a neat trick is to create another username for an existing user by creating a copy of a `passwd` entry, modifying just the username and login shell. Then you can invoke either the "normal" login shell or `arghsh` by passing the appropriate username to SSH.

Once installed on a remote system, you should be able to do tidy and worry-free SSH RPC with it like so (trivial Python example):

```python
from json import dumps
from subprocess import run

cmd = dumps(["/bin/ls", "-la", r"a file with spaces and !horrib;le$ shell meta\ charac'ters"])
run(['ssh', 'somewhere_with_arghsh', cmd])
```

Splendid.

## Creating a statically linked executable (Linux)

For Linux, Rust supports statically linking, but using musl rather than glibc. So, install that platform:

```shell
rustup target add x86_64-unknown-linux-musl
```

Then compile:

```shell
cargo build --release --target=x86_64-unknown-linux-musl
```

And there it is:

```shell
$  ldd target/x86_64-unknown-linux-musl/release/arghsh     
	statically linked
```
