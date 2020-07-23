# `memcpy`


| Library | Technique |
|---|---|
| Linux | `rep movs` |
| [Glibc](https://github.molgen.mpg.de/git-mirror/glibc/blob/master/sysdeps/x86_64/memcpy.S) | `movq` pipelined by hand (32 bits)
| | `rep movs` for 32 < _n_ < ½L1 |
| | `movq` with prefetching for ½L1 < _n_ < L1 |
| | `movntiq` with prefetching for L1 < _n_ |
| [Musl](https://github.com/esmil/musl/blob/master/src/string/x86_64/memcpy.s) | `rep movs` |
