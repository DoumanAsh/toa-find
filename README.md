# toa-find

[![Build status](https://ci.appveyor.com/api/projects/status/61ue2wyw97f17dpu/branch/master?svg=true)](https://ci.appveyor.com/project/DoumanAsh/toa-find/branch/master)
[![Build Status](https://travis-ci.org/DoumanAsh/toa-find.svg?branch=master)](https://travis-ci.org/DoumanAsh/toa-find)

GNU find replacement

Named after cute [Toa](https://vndb.org/c34928)

## Usage

```
USAGE: toa-find [options] <pattern> -- [path]..

Kawaii Toa shall find all your files recursively.

Arguments:
  <pattern> - Regular expression to filter entries by.
  [path]..  - Directory to search. By default current directory is searched.

Options:
  -h, --help         - Prints this message.
  -s, --sym          - Follow symbolic links. By default they are not followed.
      --minhop <num> - Minimum number of hops before starting to look.
      --hop <num>    - Specifies depth of recursion.

By default every type of file system entry is printed.
Below flags can be used to disable defaults and print only particular types of entries.

Entries filters:
  -d, --dir          - Prints directories.
  -f, --file         - Prints files.
```
