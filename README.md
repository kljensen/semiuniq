# Semiuniq

A [hopefully fast] uniq-like tool for removing nearby repeated lines in a file.

## Building

You need at least Rust version 1.36.0 to build `semiuniq`. You can build the
program as follows:

```bash
git clone git@github.com:kljensen/semiuniq.git
cd semiuniq
cargo build --release
```

The program will be located at `./target/release/semiuniq`.

Or, if you are a [homebrew](https://brew.sh/) user, you can install
`semiuniq` via the [kljensen/tap](https://github.com/kljensen/homebrew-tap)
tap using either

```
brew install kljensen/tap/semiuniq
```

or

```
brew tap kljensen/tap
brew install semiuniq
```


## Description

The `semiuniq` program reads over lines of input and write lines of output that
are "semi-unique" by eliminating repeated lines that are close to each other.
It is like [GNU
uniq](https://www.gnu.org/software/coreutils/manual/html_node/uniq-invocation.html)
but 1) does not require sorting the input and 2) does not guarantee global
uniqueness of output lines.

Why is this useful? It is useful because in many kinds of log files lines that
are repeated are likely to be near to each other.
For example, my shell history looks something like this

```bash
cd foo
pipenv run ansible-playbook -vvvv -i hosts.yaml playbooks/default.yaml -l hydrogen --tags unbound
vim playbooks/default.yaml
pipenv run ansible-playbook -vvvv -i hosts.yaml playbooks/default.yaml -l hydrogen --tags unbound
cd ..
tig
cd foo
pipenv run ansible-playbook -vvvv -i hosts.yaml playbooks/default.yaml -l hydrogen --tags unbound
ssh hydrogen
```

As you can see, in this session I typed the same long `ansible-playbook`
command multiple times, but it was separated by some administrative minutiae.
Now imagine if my shell history were 58,000 lines long (as it is) and I want to
find out how to run that command by searching through that history using
something like [fzf](https://github.com/junegunn/fzf). I will see many
repeated lines. I could use `sort` and `uniq` to eliminate those lines, but
that would mean sorting all 58,000 lines of history. With `semiuniq`, I make
one scan over the entire history, without sorting it, and remove repeated
lines that are "close" to each other in the history using a [least recently
used
cache](https://en.wikipedia.org/wiki/Cache_replacement_policies#Least_recently_used_(LRU))

This is useful for me. I hope it is useful for you.

## Example usage

The following sequence of shell commands shows the use of
`semiuniq` on an example file.

```
prompt> cat target/temp.txt
dog
dog
dog
dog
fish
fish
fish
fish
1
2
3
4
fish
dog
1
2
3
4
5
dog

prompt> cat target/temp.txt | semiuniq 0
dog
dog
dog
dog
fish
fish
fish
fish
1
2
3
4
fish
dog
1
2
3
4
5
dog

prompt> cat target/temp.txt | semiuniq 1
dog
fish
1
2
3
4
fish
dog
1
2
3
4
5
dog

prompt> cat target/temp.txt | semiuniq 5
dog
fish
1
2
3
4
dog
1
2
3
4
5
dog

prompt> cat target/temp.txt | semiuniq 10
dog
fish
1
2
3
4
5
```

As you can see the output of `semiuniq 0` is the same as the 
input. With `semiuniq 1`, the behavior is similar to GNU `uniq`:
a line is not printed if it is the same as the line that 
preceded it. With `semiuniq 5` a line is not printed if it
was contained in the previous 5 lines and so on. `semiuniq 1000`
would not print a line if it is identical any of the previous
1000 lines. (The hashes of lines are stored in memory, not 
the lines themselves. We're using the default hash of
[LruCache](https://github.com/jeromefroe/lru-rs), which is
[aHash](https://github.com/tkaitchuck/aHash).)

## Benchmark

This is a terrible benchmark. I used my shell
history and compared `semiuniq` to `sort | uniq`. Obviously, not sorting
saves a ton of time, duh.

```
promt> wc -l example-shell-history.txt
   54631 example-shell-history.txt

prompt> alias cmd1="cat ./example-shell-history.txt|sort|uniq >/dev/null" 

prompt> time cmd1 
cat ./example-shell-history.txt  0.00s user 0.01s system 11% cpu 0.060 total
sort  0.16s user 0.01s system 85% cpu 0.197 total
uniq > /dev/null  0.06s user 0.00s system 30% cpu 0.196 total

prompt> alias cmd2="cat ./example-shell-history.txt|semiuniq  1000 >/dev/null"

prompt> time cmd2
cat ./example-shell-history.txt  0.00s user 0.01s system 8% cpu 0.095 total
~/src/github.com/kljensen/semiuniq/target/release/semiuniq 500 > /dev/null  0.07s user 0.02s system 98% cpu 0.096 total
```

The `semiuniq` allows a few more repeated lines through with the window size
of 500.


```
prompt>  cat ./example-shell-history.txt|sort|uniq |wc -l
   54112

prompt> cat ./example-shell-history.txt|semiuniq 500 |wc -l
   54204
```


## Call for help & inspiration

If you can make this code better, please send me a pull request!  I know jack
about rust, so I had to use the duckduckgo to write this code. Most of what I
wrote here is copy/pasted from the following:

* [Read large files line by line in Rust](https://stackoverflow.com/a/45882510)
* [Reading from stdin or a file](https://stackoverflow.com/a/49964042)
* [LruCache documentation](https://docs.rs/lru/0.6.1/lru/struct.LruCache.html)

## License (the Unlicense)

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <http://unlicense.org/>

