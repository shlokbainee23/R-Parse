# Third-Party Notices

Original R-PARSE code is licensed under the MIT License in `LICENSE`. That license does not relicense the third-party material recorded here.

This notice covers source that is present in this repository, the Unicode subset compiled into the linked `tree-sitter` 0.24.7 runtime, and license text copied from the upstream files those conclusions were checked against. It does not claim that every attribution question is resolved, and it does not claim that every runtime dependency has been audited. Unresolved points are listed at the end.

## Vendored tree-sitter-r sources

Source repository: https://github.com/r-lib/tree-sitter-r

Pinned revision: `58a22794466c0fc15b0d3b40531db751593721e8` (https://github.com/r-lib/tree-sitter-r/tree/58a22794466c0fc15b0d3b40531db751593721e8)

The local files below are byte-identical to that revision. Identity was checked with git blob SHA-1. None of these local files contain a copyright or SPDX header.

| Local path | Upstream path at the pin | Git blob SHA-1 |
| --- | --- | --- |
| `implementation/crates/rparse-parser/c/parser.c` | `src/parser.c` | `9725c9b1d42f7b353edafb8e0906eeb4184462ca` |
| `implementation/crates/rparse-parser/c/scanner.c` | `src/scanner.c` | `33e5beb340f38a794e949d6fa05ede27a82ea72b` |
| `implementation/crates/rparse-parser/c/grammar.json` | `src/grammar.json` | `7a92d6f2f3fd98cb9a522d2286ef3c1122c91d93` |
| `implementation/crates/rparse-parser/c/node-types.json` | `src/node-types.json` | `c1bfd91f2388d4fee2a29c7423925edb3090e16c` |
| `implementation/crates/rparse-parser/c/tree_sitter/parser.h` | `src/tree_sitter/parser.h` | `799f599bd4e292e95565f90733c59fa422899eae` |
| `implementation/crates/rparse-parser/c/tree_sitter/array.h` | `src/tree_sitter/array.h` | `a17a574f04e0c20c38463149640badc5126063f8` |
| `implementation/crates/rparse-parser/c/tree_sitter/alloc.h` | `src/tree_sitter/alloc.h` | `1abdd1201578605b8d05a3001f01ebc0d58826d9` |

Upstream package metadata at that revision, from files that were not copied into this repository:

- `Cargo.toml` names package `tree-sitter-r` version `1.3.0`, sets `license = "MIT"`, and lists authors Kevin Ushey, Jim Hester, and Davis Vaughan.
- `package.json` names `@davisvaughan/tree-sitter-r` version `1.3.0`, sets `"license": "MIT"`, lists author Davis Vaughan, and lists devDependency `tree-sitter-cli` `^0.24.7`.

Those author fields are package metadata. They are not a substitute for the copyright line in the upstream `LICENSE`.

### Repository license

The root `LICENSE` at the pinned revision is the MIT License. Copyright line: `Copyright (c) 2025 tree-sitter-r authors`. Full text:

```
MIT License

Copyright (c) 2025 tree-sitter-r authors

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```

This is the license file upstream ships beside the matching blobs. It applies to these vendored copies. It does not name a single natural person as the copyright holder.

### Tree-sitter C headers and parser.c

`bindings/r/LICENSE.note` at the same revision was not copied as a source file. Its heading is `Tree-sitter C headers and parser.c`. The license text under that heading is identical to `LICENSE` in `tree-sitter/tree-sitter` tag `v0.24.7` (https://github.com/tree-sitter/tree-sitter/blob/v0.24.7/LICENSE). Copyright line: `Copyright (c) 2018-2024 Max Brunsfeld`.

The same three header blob SHA-1s also match these paths at that tree-sitter tag:

| Local path | tree-sitter `v0.24.7` path |
| --- | --- |
| `implementation/crates/rparse-parser/c/tree_sitter/parser.h` | `lib/src/parser.h` |
| `implementation/crates/rparse-parser/c/tree_sitter/array.h` | `cli/generate/src/templates/array.h` |
| `implementation/crates/rparse-parser/c/tree_sitter/alloc.h` | `cli/generate/src/templates/alloc.h` |

`parser.c` matches tree-sitter-r `src/parser.c` only. The local file defines `LANGUAGE_VERSION` as 14. It was not compared with a file inside the tree-sitter runtime repository, because that repository does not contain this R grammar's generated parser.

Full text of `bindings/r/LICENSE.note` at the pin:

```
Tree-sitter C headers and parser.c
--------------------------------------------------------------------------------

The MIT License (MIT)

Copyright (c) 2018-2024 Max Brunsfeld

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

`bindings/r/DESCRIPTION` at the same revision lists `Tree-sitter authors` with role `cph` and comment `Tree-sitter C headers and parser.c`. It also lists `Posit Software, PBC` with roles `cph` and `fnd` for the R package `treesitter.r`. The sources under `bindings/r/` are not in this repository. `bindings/r/LICENSE.md` uses a different copyright line, `Copyright (c) 2024 treesitter.r authors`. `bindings/r/LICENSE` is an R package stub (`YEAR: 2024`, `COPYRIGHT HOLDER: treesitter.r authors`). Those R-package files were not copied here.

The vendored headers and `parser.c` were not modified. They already match the pinned bytes. Upstream keeps the notices above beside the sources rather than inside `parser.c`.

### Linked tree-sitter crate and its Unicode subset

`implementation/Cargo.lock` resolves crates.io package `tree-sitter` version `0.24.7`, checksum `a5387dffa7ffc7d2dae12b50c6f7aab8ff79d6210147c6613561fc3d474c6f75`. `implementation/crates/rparse-parser/Cargo.toml` requires `tree-sitter` `0.24`. The crate sources are not committed in this repository. They are downloaded when the project is built.

The same version's Git tag is https://github.com/tree-sitter/tree-sitter/tree/v0.24.7. Its root `LICENSE` is the Max Brunsfeld MIT text already quoted above. The published crate's `src/unicode/LICENSE` and the tag file `lib/src/unicode/LICENSE` are the same file. Git blob SHA-1: `2e01e368768a4b992e22f98c05ead747134492a6`. Source URL: https://github.com/tree-sitter/tree-sitter/blob/v0.24.7/lib/src/unicode/LICENSE

`src/unicode/README.md` in that crate says the directory is a small subset of Unicode ICU, and that the license for those files is `src/unicode/LICENSE`. It names the copied headers `utf8.h`, `utf16.h`, and `umachine.h`, plus empty stubs `ptypes.h`, `urename.h`, and `utf.h` that those headers reference. `ICU_SHA` in the crate is `552b01f61127d30d6589aa4bf99468224979b661`. The crate directory contains those files, `LICENSE`, and `README.md`. It does not contain `cjdict.txt`, `laodict.txt`, `burmesedict.txt`, time-zone data, or double-conversion sources.

The crate's `binding_rust/build.rs` compiles `src/lib.c`. That file includes `lexer.c` and `query.c`. Both include `unicode.h`. `unicode.h` includes `unicode/utf8.h` and `unicode/utf16.h` and defines `ts_decode_utf8` and `ts_decode_utf16`, which `lexer.c` and `query.c` call. Those headers are therefore compiled into the tree-sitter library that `rparse` links. `utf8.h`, `utf16.h`, and `umachine.h` each begin with `© 2016 and later: Unicode, Inc. and others.` and `License & terms of use: http://www.unicode.org/copyright.html`. Their block comments also carry an International Business Machines copyright: `utf8.h` and `umachine.h` say 1999-2015; `utf16.h` says 1999-2012.

#### Source checkout and compiled distribution

A source checkout of this repository does not contain the ICU headers or `src/unicode/LICENSE`. Publishing that source tree does not itself copy those files. The Unicode notice below is included here because a build of this project compiles them into the linked runtime, and because this file is the associated documentation for that runtime.

A compiled `rparse` executable does contain that compiled software. `Dockerfile` builds `target/release/rparse` and copies that executable into the runtime image at `/usr/local/bin/rparse`. The runtime stage also copies the root `LICENSE` to `/app/LICENSE` and this file to `/app/THIRD_PARTY_NOTICES.md`, so those notices stay in the final image with the deployed program. The opening Unicode notice allows the copyright and permission notice to appear either with the copies or in associated documentation. The IBM notice quoted next requires its copyright and permission notice in all copies of that software and in supporting documentation. This file supplies that text for the source repository and for the runtime image.

#### Applicable text from `src/unicode/LICENSE`

The following text is the opening notice in that file.

```
COPYRIGHT AND PERMISSION NOTICE (ICU 58 and later)

Copyright © 1991-2019 Unicode, Inc. All rights reserved.
Distributed under the Terms of Use in https://www.unicode.org/copyright.html.

Permission is hereby granted, free of charge, to any person obtaining
a copy of the Unicode data files and any associated documentation
(the "Data Files") or Unicode software and any associated documentation
(the "Software") to deal in the Data Files or Software
without restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, and/or sell copies of
the Data Files or Software, and to permit persons to whom the Data Files
or Software are furnished to do so, provided that either
(a) this copyright and permission notice appear with all copies
of the Data Files or Software, or
(b) this copyright and permission notice appear in associated
Documentation.

THE DATA FILES AND SOFTWARE ARE PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE
WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT OF THIRD PARTY RIGHTS.
IN NO EVENT SHALL THE COPYRIGHT HOLDER OR HOLDERS INCLUDED IN THIS
NOTICE BE LIABLE FOR ANY CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL
DAMAGES, OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THE DATA FILES OR SOFTWARE.

Except as contained in this notice, the name of a copyright holder
shall not be used in advertising or otherwise to promote the sale,
use or other dealings in these Data Files or Software without prior
written authorization of the copyright holder.
```

The same file then contains a third-party section. The part quoted next is the introduction and item 1, the ICU license for ICU 1.8.1 through ICU 57.1. It is included because the copied headers carry the IBM copyright lines above.

```
Third-Party Software Licenses

This section contains third-party software notices and/or additional
terms for licensed third-party software components included within ICU
libraries.

1. ICU License - ICU 1.8.1 to ICU 57.1

COPYRIGHT AND PERMISSION NOTICE

Copyright (c) 1995-2016 International Business Machines Corporation and others
All rights reserved.

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, and/or sell copies of the Software, and to permit persons
to whom the Software is furnished to do so, provided that the above
copyright notice(s) and this permission notice appear in all copies of
the Software and that both the above copyright notice(s) and this
permission notice appear in supporting documentation.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT
OF THIRD PARTY RIGHTS. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
HOLDERS INCLUDED IN THIS NOTICE BE LIABLE FOR ANY CLAIM, OR ANY
SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES WHATSOEVER
RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF
CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

Except as contained in this notice, the name of a copyright holder
shall not be used in advertising or otherwise to promote the sale, use
or other dealings in this Software without prior written authorization
of the copyright holder.

All trademarks and registered trademarks mentioned herein are the
property of their respective owners.
```

The same upstream file continues with these later headings:

- `2. Chinese/Japanese Word Break Dictionary Data (cjdict.txt)`
- `3. Lao Word Break Dictionary Data (laodict.txt)`
- `4. Burmese Word Break Dictionary Data (burmesedict.txt)`
- `5. Time Zone Database`
- `6. Google double-conversion`

Those sections were read in the upstream file. The files they name are not in the tree-sitter 0.24.7 `src/unicode` directory, and they are not among the sources `lib.c` compiles. Their notice text is not reproduced here. The complete file remains at the URL and blob above.

## What was audited

License text was read and, where it applies to material this project vendors or compiles in, copied above for:

- the vendored tree-sitter-r files at `58a22794466c0fc15b0d3b40531db751593721e8`, including that revision's root `LICENSE` and `bindings/r/LICENSE.note`
- `tree-sitter` tag `v0.24.7` root `LICENSE`, which matches the license body of that `LICENSE.note`
- `tree-sitter` 0.24.7 `src/unicode/LICENSE`, matched between the Git tag and the crates.io crate named in `implementation/Cargo.lock`

This is not a complete dependency-license audit. These other crates are recorded in `implementation/Cargo.lock` and were not opened for license text.

Reachable from the lockfile as normal dependencies of `tree-sitter` 0.24.7 or of an R-PARSE crate, and therefore able to ship inside a compiled `rparse` executable. The built executable was not inspected:

- `tree-sitter-language` 0.1.8
- `streaming-iterator` 0.1.9
- `regex` 1.13.1
- `regex-automata` 0.4.18
- `regex-syntax` 0.8.11
- `aho-corasick` 1.1.5
- `memchr` 2.8.3
- `thiserror` 1.0.69

`tree-sitter`'s Rust sources import `streaming-iterator` and `tree_sitter_language` and construct `regex::bytes::Regex`. `thiserror` is a dependency of `rparse-core`. None of those license files were read.

Present only as build or proc-macro dependencies in the lockfile, judging by the crate manifests rather than by inspecting a built binary:

- `cc` 1.6.0, a build dependency of `rparse-parser` and of `tree-sitter`, and its dependencies `find-msvc-tools` 0.1.14 and `shlex` 2.0.1
- `thiserror-impl` 1.0.69, `syn` 2.0.119, `quote` 1.0.47, `proc-macro2` 1.0.107, and `unicode-ident` 1.0.26

The Docker image's operating-system packages were not audited. `Dockerfile` uses `rust:1.80-slim-bookworm` to build and `debian:bookworm-slim` to run, and it installs `ca-certificates` and `curl`. `wasmtime-c-api` is an optional tree-sitter feature and is not in `implementation/Cargo.lock`.

## Unresolved

1. `parser.c` is covered by the tree-sitter-r root `LICENSE` (copyright line: tree-sitter-r authors, 2025) and is also named by `bindings/r/LICENSE.note` (copyright line: Max Brunsfeld, 2018-2024). Both texts are preserved above. This audit does not treat one notice as replacing the other. The generated parse tables come from the tree-sitter-r grammar. The header files match tree-sitter generator and runtime sources.

2. `scanner.c`, `grammar.json`, and `node-types.json` match tree-sitter-r paths and are not named in `LICENSE.note`. They are included under the root `LICENSE` quoted above. `grammar.json` and `node-types.json` are generated artifacts. The upstream files inspected here do not say whether the tree-sitter generator's authors claim a separate copyright in those JSON files.

3. `scanner.c` comments that its stack structure was inspired by tree-sitter-julia. No tree-sitter-julia source is vendored here, and the pinned tree-sitter-r `LICENSE` does not include a tree-sitter-julia license. A line-by-line comparison with tree-sitter-julia was not done.

4. Upstream names additional copyright holders for the R package binding (`treesitter.r authors`, 2024, and `Posit Software, PBC` as `cph`). That binding is not vendored. Those lines are not used here as the copyright line of the C sources that were copied. The copyright line attached to the copied sources by the repository root `LICENSE` remains `Copyright (c) 2025 tree-sitter-r authors`, together with the separate header and `parser.c` notice above.

5. The crates and operating-system packages listed under "What was audited" as not opened remain unresolved. No claim is made that their licenses match the notices copied above.

6. This repository had no `LICENSE`, `COPYING`, or `NOTICE` file before the R-PARSE MIT license was added. A search of project source for copyright headers did not find one. That search does not prove that every third-party snippet is absent.
