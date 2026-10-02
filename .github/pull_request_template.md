## Outreachy Application Process

These instructions are specifically for Outreachy Applicants in October 2026. 

IMPORTANT: You **must** only open one pull request (PR).

### Banned AI Usage

**Do not** use AI to write GitHub descriptions, GitHub comments, code comments, or Zulip chats. Write in your native language, or at your current English level. In your PR description, you must tell us how you used AI. You must review code yourself before you open a PR.

For the best learning experience, write the code yourself, without using AI.

Applicants must follow the [Rust Project AI/LLM policy][llm-policy].

[llm-policy]: https://blog.rust-lang.org/inside-rust/2026/08/05/rust-langrust-is-adopting-an-llm-policy/

### Application Steps

Claim a C++ standard library type from the list [in the tracking issue][tracking-issue] by clicking on that issue, then commenting on it. This is the C++ type you will be working on. Before you claim a ticket, make sure no-one else has commented before you.

After each step, push your work to your open PR, make sure CI passes, then **wait** for 2-4 days for a mentor to review it. If you need help, post a message in the [#outreachy > Overloading Applicant Help Oct 2026][outreachy-zulip] topic on Zulip.

This coding task focuses on Rust and C++ overloads. It is ok to skip drop/destructors (leak memory), use unsafe, and ignore some compiler warnings. Shortcuts and workarounds can be used to simplify non-overload code.

[tracking-issue]: https://github.com/rustfoundation/overloading-macros/issues/19
[outreachy-zulip]: https://rust-lang.zulipchat.com/#narrow/channel/578347-outreachy/topic/Overloading.20Applicant.20Help.20Oct.202026/with/628605857.01

1. Write Rust wrappers for 3-5 C++ overloads on that type:
    - [ ] Create a new file named after your type in [overloading-macros/cpp-overload-test/src/stdcpp](https://github.com/rustfoundation/overloading-macros/tree/main/cpp-overload-test/src/stdcpp)
    - [ ] Wrap constructors first, writing tests to make sure the correct constructor is called
    - [ ] Explain any potential compatibility issues in a `Limitations` doc section 
    - [ ] Explain usability issues in an `Ergonomics` doc section
    - [ ] Push your changes and wait for a review
2. Add a separate example with some failing overloads
    - [ ] Create a new file named after your type in [overloading-macros/cpp-overload-test/examples](https://github.com/rustfoundation/overloading-macros/tree/main/cpp-overload-test/examples)
    - [ ] Wrap accessor/modifier methods 
    - [ ] Explain how they fail in an `Incompatibilities` doc section
    - [ ] Explain how the overload failures could be resolved in a `Workarounds` doc section
    - [ ] Optional: If there's a C++ feature with no equivalent in Rust, explain that in an `Inexpressible Overloads` doc section
    - [ ] Push your changes and wait for a review
3. Update the [overload macro compatibility table Markdown doc](https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overloads-in-rust.md)
    - [ ] If you haven't checked some kinds of overloads, put a ？in those squares
    - [ ] Push your changes and wait for a review

Mark items in this checklist as done by clicking the box next to each item.

#### Focus on Interesting Overloads

Only wrap functions that have overloads. (It is ok to wrap non-overloaded functions just for testing.)

Ignore these overloads:
- overloaded operators: they are already possible in Rust
- overloads with an allocator argument, or that only differ by `noexcept`: not an interesting difference
- assignment functions, assignment operators, and `swap`: no equivalent in Rust (which uses memcpy for assignment/swaps)

Only code one example of overloads that:
- only differ by `const`/non-`const`, `&`/`&&` (or `&`/`&mut`) 
- are iterator methods that only differ by position (`before_begin`/`begin`,`end`)

You can look at [stdcpp/forward_list.rs (working overloads)](https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/src/stdcpp/forward_list.rs) and [stdcpp-forward-list.rs (failing overloads)](https://github.com/rustfoundation/overloading-macros/blob/main/cpp-overload-test/examples/stdcpp-forward-list.rs) for example code.
