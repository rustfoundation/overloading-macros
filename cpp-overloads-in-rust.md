
# C++ Overloads in Rust: Experimental Outcomes

***This document is an initial, partially reviewed draft***

This table shows how well C++ standard library overloads can be represented in Rust.

These outcomes are based on the current state of the experimental `splat` compiler feature and
overloading macro: they are not Rust language design decisions.

See the Limitations section below the table for more details.

*Key:*<br/>
⛔ Conflicting Overloads<br/>
🦀 Rust Compiler Limitations<br/>
⚙️ Overload Macro Limitations<br/>
🌗 Partial Support or Poor Ergonomics<br/>
✅ Works in Rust<br/>
？ Needs Analysis<br/>
*blank* No C++ Overloads<br/>

| C++ Standard Library Class | Constructors | Accessors | Modifiers  | Transforms |
| :------------------------- | :----------- | :-------- | :--------- | :--------- |
| *Free Functions*           | Constructors | Accessors | Modifiers  | Transforms |
| TODO                       |              | ？        |  ？        | ？         |
| *Non-Template Classes*     | Constructors | Accessors | Modifiers  | Transforms |
| TODO                       | ？           | ？        |  ？        | ？         |
| *Template Classes*         | Constructors | Accessors | Modifiers  | Transforms |
| [forward_list]             | ✅           | 🌗⚙️      | 🌗⚙️？[^1] | ✅         |
| TODO                       | ？           | ？        |  ？        | ？         |

Constructors: Create an instance of the class, called as `Class::Class(...)`<br/>
Accessors: Get information about the class, called as `instance->get(...)`<br/>
Modifiers: Add or change the items in a list class, called as `instance->put(...)`<br/>
Transforms: Rearrange the items in a list class, called as `instance->sort(...)`<br/>

[forward_list]: https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/src/stdcpp/forward_list.rs

[^1]: Mostly caused by the differences between Rust and C++ iterators, could be overcome with type wrappers.

## Limitations

This section summarises common issues discovered during the analysis, and documents less
interesting overloads that were skipped.

### Typical Overload Conflicts

Overload macro missing functionality:

- methods that only differ by `self` type, for example `get(&self) -> &T` and
  `get(&mut self) -> &mut T`
  - this can be worked around using associated functions, but the ergonomics suffer
- Generic types:
  - this makes representing C++ templated types with generic Rust types impossible
  - doesn't support `impl Iterator` or `where I: Iterator` as arguments, but supports
    `&dyn Iterator` (which is less ergonomic)
- Lifetimes:
  - generic lifetimes aren't supported
  - when the input method has elided lifetimes, the generated code doesn't add lifetimes where
    needed

### Ergonomics

Some overloads are confusing or hard to use, so we might not want to represent them in Rust:

- `get(&self) -> &T` and `get(&mut self) -> &mut T` is a subtle difference, typical Rust APIs use
  `get` and `get_mut` instead
- in generic list overloads, `new(count: usize)` and `new(item: T)` are easily confused if `T` is
  an integer (and they conflict if `T` is `usize`, which means it can't be fully generic)
  - this can be resolved by skipping overloads for initializer lists
- `&dyn Iterator` overloads require a type cast
  - this could be resolved with improvements to the macro, or with an iterator adapter type

Type inference doesn't work on some overloads: this is likely a macro or compiler experiment limitation.

Some C++ overloads [use a tag argument][cpp-tag-arg] for [overload disambiguation][cpp-tag-type],
a strategy which could be adopted in Rust APIs. But in some cases, the Rust overload
[might not need disambiguation][rust-no-disambig].

[cpp-tag-arg]: https://en.cppreference.com/cpp/container/forward_list/forward_list
[cpp-tag-type]: https://en.cppreference.com/cpp/ranges/from_range
[rust-no-disambig]:  https://github.com/rustfoundation/overloading-macros/blob/52c3f3aa5e4e6f11d71d2d832fce733f00d0b087/cpp-overload-test/src/stdcpp/forward_list.rs#L89

### Scope

Some overloads have been ignored or minimised in this analysis.

These overloads are less interesting:

- overloaded operators: already available in stable Rust
- overloads with an allocator argument, or that only differ by noexcept: not an interesting
  difference
- overloads that only differ by `const`/non-`const`, `&`/`&&` (lvalue/rvalue) in C++, or
  `&`/`&mut` in Rust

These overloads have no equivalent in Rust:

- assignment functions, assignment operators, and `swap`: there is no "type move method" in Rust
  (it uses `memcpy` for assignment/swaps)
- legacy iterator pointer-like methods for various positions
  (`before_begin`/`begin`/`end`/`after_end`)
- initializer list methods: these are also a common source of overload conflicts for C++ templated
  lists

#### Equivalences & Supersets

When the same overloaded functions are available on different classes, we only analyse the overload
on one of those classes.

When one overloaded function takes the same argument sets (or a subset of the argument set) of
another overloaded function, we only analyse one of the argument sets. For example,
`unique()`/`unique(predicate)` and `sort()`/`sort(predicate)`.

In some cases, we focus on the most "difficult" overload in a category, and skip the rest.
