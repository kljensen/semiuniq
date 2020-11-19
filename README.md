# Semiuniq - a fast uniq-like tool for removing nearby duplicated lines in a file

The `semiuniq` program reads over lines of input and write lines of output that
are "semi-unique" by eliminating duplicate lines that are close to each other.
It is like [GNU
uniq](https://www.gnu.org/software/coreutils/manual/html_node/uniq-invocation.html)
but 1) does not require sorting the input and 2) does not guarantee global
uniqueness of output lines.

Why is this useful? It is useful because in many kinds of files the lines of
the file are highly correlated such that lines that are identical are likely to
be next to each other. For example, my shell history looks somethign like this

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
Now imagine if my shell history were 58,000 lines long (it is) and I want to
find out how to run that command by searching through that history using
something like [fzf](https://github.com/junegunn/fzf), I will see many
duplicated lines. I could use `sort` and `uniq` to eliminate those lines, but
that would mean sorting all 58,000 lines of history. With `semiuniq`, I make
one scan over the entire history, without sorting it, and remove duplicate
lines that are "close" to each other in the history using a [least recently
used
cache](https://en.wikipedia.org/wiki/Cache_replacement_policies#Least_recently_used_(LRU))

This is useful for me. I hope it is useful for you.

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

