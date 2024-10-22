 Cargo Workspaces - The Rust Programming Language

 var path\_to\_root = ""; var default\_theme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "navy" : "light";  try { var theme = localStorage.getItem('mdbook-theme'); var sidebar = localStorage.getItem('mdbook-sidebar'); if (theme.startsWith('"') && theme.endsWith('"')) { localStorage.setItem('mdbook-theme', theme.slice(1, theme.length - 1)); } if (sidebar.startsWith('"') && sidebar.endsWith('"')) { localStorage.setItem('mdbook-sidebar', sidebar.slice(1, sidebar.length - 1)); } } catch (e) { }  var theme; try { theme = localStorage.getItem('mdbook-theme'); } catch(e) { } if (theme === null || theme === undefined) { theme = default\_theme; } var html = document.querySelector('html'); html.classList.remove('light') html.classList.add(theme); var body = document.querySelector('body'); body.classList.remove('no-js') body.classList.add('js');  var body = document.querySelector('body'); var sidebar = null; var sidebar\_toggle = document.getElementById("sidebar-toggle-anchor"); if (document.body.clientWidth \>= 1080) { try { sidebar = localStorage.getItem('mdbook-sidebar'); } catch(e) { } sidebar = sidebar || 'visible'; } else { sidebar = 'hidden'; } sidebar\_toggle.checked = sidebar === 'visible'; body.classList.remove('sidebar-visible'); body.classList.add("sidebar-" + sidebar);

1. [The Rust Programming Language](title-page.html)
2. [Foreword](foreword.html)
3. [Introduction](ch00-00-introduction.html)
4. [**1.** Getting Started](ch01-00-getting-started.html)
5. 1. [**1.1.** Installation](ch01-01-installation.html)
   2. [**1.2.** Hello, World!](ch01-02-hello-world.html)
   3. [**1.3.** Hello, Cargo!](ch01-03-hello-cargo.html)

6. [**2.** Programming a Guessing Game](ch02-00-guessing-game-tutorial.html)
7. [**3.** Common Programming Concepts](ch03-00-common-programming-concepts.html)
8. 1. [**3.1.** Variables and Mutability](ch03-01-variables-and-mutability.html)
   2. [**3.2.** Data Types](ch03-02-data-types.html)
   3. [**3.3.** Functions](ch03-03-how-functions-work.html)
   4. [**3.4.** Comments](ch03-04-comments.html)
   5. [**3.5.** Control Flow](ch03-05-control-flow.html)

9. [**4.** Understanding Ownership](ch04-00-understanding-ownership.html)
10. 1. [**4.1.** What is Ownership?](ch04-01-what-is-ownership.html)
   2. [**4.2.** References and Borrowing](ch04-02-references-and-borrowing.html)
   3. [**4.3.** The Slice Type](ch04-03-slices.html)

11. [**5.** Using Structs to Structure Related Data](ch05-00-structs.html)
12. 1. [**5.1.** Defining and Instantiating Structs](ch05-01-defining-structs.html)
   2. [**5.2.** An Example Program Using Structs](ch05-02-example-structs.html)
   3. [**5.3.** Method Syntax](ch05-03-method-syntax.html)

13. [**6.** Enums and Pattern Matching](ch06-00-enums.html)
14. 1. [**6.1.** Defining an Enum](ch06-01-defining-an-enum.html)
   2. [**6.2.** The match Control Flow Construct](ch06-02-match.html)
   3. [**6.3.** Concise Control Flow with if let](ch06-03-if-let.html)

15. [**7.** Managing Growing Projects with Packages, Crates, and Modules](ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
16. 1. [**7.1.** Packages and Crates](ch07-01-packages-and-crates.html)
   2. [**7.2.** Defining Modules to Control Scope and Privacy](ch07-02-defining-modules-to-control-scope-and-privacy.html)
   3. [**7.3.** Paths for Referring to an Item in the Module Tree](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
   4. [**7.4.** Bringing Paths Into Scope with the use Keyword](ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)
   5. [**7.5.** Separating Modules into Different Files](ch07-05-separating-modules-into-different-files.html)

17. [**8.** Common Collections](ch08-00-common-collections.html)
18. 1. [**8.1.** Storing Lists of Values with Vectors](ch08-01-vectors.html)
   2. [**8.2.** Storing UTF-8 Encoded Text with Strings](ch08-02-strings.html)
   3. [**8.3.** Storing Keys with Associated Values in Hash Maps](ch08-03-hash-maps.html)

19. [**9.** Error Handling](ch09-00-error-handling.html)
20. 1. [**9.1.** Unrecoverable Errors with panic!](ch09-01-unrecoverable-errors-with-panic.html)
   2. [**9.2.** Recoverable Errors with Result](ch09-02-recoverable-errors-with-result.html)
   3. [**9.3.** To panic! or Not to panic!](ch09-03-to-panic-or-not-to-panic.html)

21. [**10.** Generic Types, Traits, and Lifetimes](ch10-00-generics.html)
22. 1. [**10.1.** Generic Data Types](ch10-01-syntax.html)
   2. [**10.2.** Traits: Defining Shared Behavior](ch10-02-traits.html)
   3. [**10.3.** Validating References with Lifetimes](ch10-03-lifetime-syntax.html)

23. [**11.** Writing Automated Tests](ch11-00-testing.html)
24. 1. [**11.1.** How to Write Tests](ch11-01-writing-tests.html)
   2. [**11.2.** Controlling How Tests Are Run](ch11-02-running-tests.html)
   3. [**11.3.** Test Organization](ch11-03-test-organization.html)

25. [**12.** An I/O Project: Building a Command Line Program](ch12-00-an-io-project.html)
26. 1. [**12.1.** Accepting Command Line Arguments](ch12-01-accepting-command-line-arguments.html)
   2. [**12.2.** Reading a File](ch12-02-reading-a-file.html)
   3. [**12.3.** Refactoring to Improve Modularity and Error Handling](ch12-03-improving-error-handling-and-modularity.html)
   4. [**12.4.** Developing the Library’s Functionality with Test Driven Development](ch12-04-testing-the-librarys-functionality.html)
   5. [**12.5.** Working with Environment Variables](ch12-05-working-with-environment-variables.html)
   6. [**12.6.** Writing Error Messages to Standard Error Instead of Standard Output](ch12-06-writing-to-stderr-instead-of-stdout.html)

27. [**13.** Functional Language Features: Iterators and Closures](ch13-00-functional-features.html)
28. 1. [**13.1.** Closures: Anonymous Functions that Capture Their Environment](ch13-01-closures.html)
   2. [**13.2.** Processing a Series of Items with Iterators](ch13-02-iterators.html)
   3. [**13.3.** Improving Our I/O Project](ch13-03-improving-our-io-project.html)
   4. [**13.4.** Comparing Performance: Loops vs. Iterators](ch13-04-performance.html)

29. [**14.** More about Cargo and Crates.io](ch14-00-more-about-cargo.html)
30. 1. [**14.1.** Customizing Builds with Release Profiles](ch14-01-release-profiles.html)
   2. [**14.2.** Publishing a Crate to Crates.io](ch14-02-publishing-to-crates-io.html)
   3. [**14.3.** Cargo Workspaces](ch14-03-cargo-workspaces.html)
   4. [**14.4.** Installing Binaries from Crates.io with cargo install](ch14-04-installing-binaries.html)
   5. [**14.5.** Extending Cargo with Custom Commands](ch14-05-extending-cargo.html)

31. [**15.** Smart Pointers](ch15-00-smart-pointers.html)
32. 1. [**15.1.** Using Box\<T\> to Point to Data on the Heap](ch15-01-box.html)
   2. [**15.2.** Treating Smart Pointers Like Regular References with the Deref Trait](ch15-02-deref.html)
   3. [**15.3.** Running Code on Cleanup with the Drop Trait](ch15-03-drop.html)
   4. [**15.4.** Rc\<T\>, the Reference Counted Smart Pointer](ch15-04-rc.html)
   5. [**15.5.** RefCell\<T\> and the Interior Mutability Pattern](ch15-05-interior-mutability.html)
   6. [**15.6.** Reference Cycles Can Leak Memory](ch15-06-reference-cycles.html)

33. [**16.** Fearless Concurrency](ch16-00-concurrency.html)
34. 1. [**16.1.** Using Threads to Run Code Simultaneously](ch16-01-threads.html)
   2. [**16.2.** Using Message Passing to Transfer Data Between Threads](ch16-02-message-passing.html)
   3. [**16.3.** Shared-State Concurrency](ch16-03-shared-state.html)
   4. [**16.4.** Extensible Concurrency with the Sync and Send Traits](ch16-04-extensible-concurrency-sync-and-send.html)

35. [**17.** Object Oriented Programming Features of Rust](ch17-00-oop.html)
36. 1. [**17.1.** Characteristics of Object-Oriented Languages](ch17-01-what-is-oo.html)
   2. [**17.2.** Using Trait Objects That Allow for Values of Different Types](ch17-02-trait-objects.html)
   3. [**17.3.** Implementing an Object-Oriented Design Pattern](ch17-03-oo-design-patterns.html)

37. [**18.** Patterns and Matching](ch18-00-patterns.html)
38. 1. [**18.1.** All the Places Patterns Can Be Used](ch18-01-all-the-places-for-patterns.html)
   2. [**18.2.** Refutability: Whether a Pattern Might Fail to Match](ch18-02-refutability.html)
   3. [**18.3.** Pattern Syntax](ch18-03-pattern-syntax.html)

39. [**19.** Advanced Features](ch19-00-advanced-features.html)
40. 1. [**19.1.** Unsafe Rust](ch19-01-unsafe-rust.html)
   2. [**19.2.** Advanced Traits](ch19-03-advanced-traits.html)
   3. [**19.3.** Advanced Types](ch19-04-advanced-types.html)
   4. [**19.4.** Advanced Functions and Closures](ch19-05-advanced-functions-and-closures.html)
   5. [**19.5.** Macros](ch19-06-macros.html)

41. [**20.** Final Project: Building a Multithreaded Web Server](ch20-00-final-project-a-web-server.html)
42. 1. [**20.1.** Building a Single-Threaded Web Server](ch20-01-single-threaded.html)
   2. [**20.2.** Turning Our Single-Threaded Server into a Multithreaded Server](ch20-02-multithreaded.html)
   3. [**20.3.** Graceful Shutdown and Cleanup](ch20-03-graceful-shutdown-and-cleanup.html)

43. [**21.** Appendix](appendix-00.html)
44. 1. [**21.1.** A - Keywords](appendix-01-keywords.html)
   2. [**21.2.** B - Operators and Symbols](appendix-02-operators.html)
   3. [**21.3.** C - Derivable Traits](appendix-03-derivable-traits.html)
   4. [**21.4.** D - Useful Development Tools](appendix-04-useful-development-tools.html)
   5. [**21.5.** E - Editions](appendix-05-editions.html)
   6. [**21.6.** F - Translations of the Book](appendix-06-translation.html)
   7. [**21.7.** G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.html)

 var sidebarScrollbox = document.querySelector('#sidebar .sidebar-scrollbox'); sidebarScrollbox.addEventListener('click', function(e) { if (e.target.tagName === 'A') { sessionStorage.setItem('sidebar-scroll', sidebarScrollbox.scrollTop); } }, { passive: true }); var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll'); sessionStorage.removeItem('sidebar-scroll'); if (sidebarScrollTop) { // preserve sidebar scroll position when navigating via links within sidebar sidebarScrollbox.scrollTop = sidebarScrollTop; } else { // scroll sidebar to current active section when navigating via "next/previous chapter" buttons var activeSection = document.querySelector('#sidebar .active'); if (activeSection) { activeSection.scrollIntoView({ block: 'center' }); } }

* Light
* Rust
* Coal
* Navy
* Ayu

The Rust Programming Language
==========

[](print.html) [](https://github.com/rust-lang/book)

 document.getElementById('sidebar-toggle').setAttribute('aria-expanded', sidebar === 'visible'); document.getElementById('sidebar').setAttribute('aria-hidden', sidebar !== 'visible'); Array.from(document.querySelectorAll('#sidebar a')).forEach(function(link) { link.setAttribute('tabIndex', sidebar === 'visible' ? 0 : -1); });

[Cargo Workspaces](#cargo-workspaces)
----------

In Chapter 12, we built a package that included a binary crate and a library
crate. As your project develops, you might find that the library crate
continues to get bigger and you want to split your package further into
multiple library crates. Cargo offers a feature called *workspaces* that can
help manage multiple related packages that are developed in tandem.

### [Creating a Workspace](#creating-a-workspace) ###

A *workspace* is a set of packages that share the same *Cargo.lock* and output
directory. Let’s make a project using a workspace—we’ll use trivial code so we
can concentrate on the structure of the workspace. There are multiple ways to
structure a workspace, so we’ll just show one common way. We’ll have a
workspace containing a binary and two libraries. The binary, which will provide
the main functionality, will depend on the two libraries. One library will
provide an `add_one` function, and a second library an `add_two` function.
These three crates will be part of the same workspace. We’ll start by creating
a new directory for the workspace:

```
$ mkdir add
$ cd add

```

Next, in the *add* directory, we create the *Cargo.toml* file that will
configure the entire workspace. This file won’t have a `[package]` section.
Instead, it will start with a `[workspace]` section that will allow us to add
members to the workspace by specifying the path to the package with our binary
crate; in this case, that path is *adder*:

Filename: Cargo.toml

```
[workspace]

members = [
    "adder",
]

```

Next, we’ll create the `adder` binary crate by running `cargo new` within the*add* directory:

```
$ cargo new adder
     Created binary (application) `adder` package

```

At this point, we can build the workspace by running `cargo build`. The files
in your *add* directory should look like this:

```
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target

```

The workspace has one *target* directory at the top level that the compiled
artifacts will be placed into; the `adder` package doesn’t have its own*target* directory. Even if we were to run `cargo build` from inside the*adder* directory, the compiled artifacts would still end up in *add/target*rather than *add/adder/target*. Cargo structures the *target* directory in a
workspace like this because the crates in a workspace are meant to depend on
each other. If each crate had its own *target* directory, each crate would have
to recompile each of the other crates in the workspace to place the artifacts
in its own *target* directory. By sharing one *target* directory, the crates
can avoid unnecessary rebuilding.

### [Creating the Second Package in the Workspace](#creating-the-second-package-in-the-workspace) ###

Next, let’s create another member package in the workspace and call it`add_one`. Change the top-level *Cargo.toml* to specify the *add\_one* path in
the `members` list:

Filename: Cargo.toml

```
[workspace]

members = [
    "adder",
    "add_one",
]

```

Then generate a new library crate named `add_one`:

```
$ cargo new add_one --lib
     Created library `add_one` package

```

Your *add* directory should now have these directories and files:

```
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target

```

In the *add\_one/src/lib.rs* file, let’s add an `add_one` function:

Filename: add\_one/src/lib.rs

```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Now we can have the `adder` package with our binary depend on the `add_one`package that has our library. First, we’ll need to add a path dependency on`add_one` to *adder/Cargo.toml*.

Filename: adder/Cargo.toml

```
[dependencies]
add_one = { path = "../add_one" }

```

Cargo doesn’t assume that crates in a workspace will depend on each other, so
we need to be explicit about the dependency relationships.

Next, let’s use the `add_one` function (from the `add_one` crate) in the`adder` crate. Open the *adder/src/main.rs* file and add a `use` line at the
top to bring the new `add_one` library crate into scope. Then change the `main`function to call the `add_one` function, as in Listing 14-7.

Filename: adder/src/main.rs

```
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
```

Listing 14-7: Using the `add_one` library crate from the`adder` crate

Let’s build the workspace by running `cargo build` in the top-level *add*directory!

```
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s

```

To run the binary crate from the *add* directory, we can specify which
package in the workspace we want to run by using the `-p` argument and the
package name with `cargo run`:

```
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!

```

This runs the code in *adder/src/main.rs*, which depends on the `add_one` crate.

#### [Depending on an External Package in a Workspace](#depending-on-an-external-package-in-a-workspace) ####

Notice that the workspace has only one *Cargo.lock* file at the top level,
rather than having a *Cargo.lock* in each crate’s directory. This ensures that
all crates are using the same version of all dependencies. If we add the `rand`package to the *adder/Cargo.toml* and *add\_one/Cargo.toml* files, Cargo will
resolve both of those to one version of `rand` and record that in the one*Cargo.lock*. Making all crates in the workspace use the same dependencies
means the crates will always be compatible with each other. Let’s add the`rand` crate to the `[dependencies]` section in the *add\_one/Cargo.toml* file
so we can use the `rand` crate in the `add_one` crate:

Filename: add\_one/Cargo.toml

```
[dependencies]
rand = "0.8.5"

```

We can now add `use rand;` to the *add\_one/src/lib.rs* file, and building the
whole workspace by running `cargo build` in the *add* directory will bring in
and compile the `rand` crate. We will get one warning because we aren’t
referring to the `rand` we brought into scope:

```
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s

```

The top-level *Cargo.lock* now contains information about the dependency of`add_one` on `rand`. However, even though `rand` is used somewhere in the
workspace, we can’t use it in other crates in the workspace unless we add`rand` to their *Cargo.toml* files as well. For example, if we add `use rand;`to the *adder/src/main.rs* file for the `adder` package, we’ll get an error:

```
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`

```

To fix this, edit the *Cargo.toml* file for the `adder` package and indicate
that `rand` is a dependency for it as well. Building the `adder` package will
add `rand` to the list of dependencies for `adder` in *Cargo.lock*, but no
additional copies of `rand` will be downloaded. Cargo will ensure that every
crate in every package in the workspace using the `rand` package will be using
the same version as long as they specify compatible versions of `rand`, saving
us space and ensuring that the crates in the workspace will be compatible with
each other.

If crates in the workspace specify incompatible versions of the same dependency,
Cargo will resolve each of them, but will still try to resolve as few versions
as possible.

#### [Adding a Test to a Workspace](#adding-a-test-to-a-workspace) ####

For another enhancement, let’s add a test of the `add_one::add_one` function
within the `add_one` crate:

Filename: add\_one/src/lib.rs

```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

Now run `cargo test` in the top-level *add* directory. Running `cargo test` in
a workspace structured like this one will run the tests for all the crates in
the workspace:

```
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/add_one-f0253159197f7841)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-49979ff40686fa8e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

The first section of the output shows that the `it_works` test in the `add_one`crate passed. The next section shows that zero tests were found in the `adder`crate, and then the last section shows zero documentation tests were found in
the `add_one` crate.

We can also run tests for one particular crate in a workspace from the
top-level directory by using the `-p` flag and specifying the name of the crate
we want to test:

```
$ cargo test -p add_one
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-b3235fea9a156f74)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

This output shows `cargo test` only ran the tests for the `add_one` crate and
didn’t run the `adder` crate tests.

If you publish the crates in the workspace to [crates.io](https://crates.io/),
each crate in the workspace will need to be published separately. Like `cargo test`, we can publish a particular crate in our workspace by using the `-p`flag and specifying the name of the crate we want to publish.

For additional practice, add an `add_two` crate to this workspace in a similar
way as the `add_one` crate!

As your project grows, consider using a workspace: it’s easier to understand
smaller, individual components than one big blob of code. Furthermore, keeping
the crates in a workspace can make coordination between crates easier if they
are often changed at the same time.

[](ch14-02-publishing-to-crates-io.html) [](ch14-04-installing-binaries.html)

[](ch14-02-publishing-to-crates-io.html) [](ch14-04-installing-binaries.html)

 window.playground\_copyable = true;