# Lokinit

> The smallest cross-platform native windowing library.

[![Discord](https://img.shields.io/discord/1092477105079595038?color=7289DA&label=%20&logo=discord&logoColor=white)](https://discord.gg/D5pzrmyqz3)

Lokinit is the smallest cross-platform native windowing library for Rust. Its goal is to bring a window to the screen on all major operating systems, with a dependency tree as minimal as it should be, and code that is simple and straightforward.

Cargo may be an amazing package manager, but making projects with hundreds of dependencies nested in a deep tree has some disadvantages:
- it clogs up compile times by orders of magnitude. Upon a `cargo clean`, it's very easy to see middle-sized projects take several minutes to build. Miniquad, a very good example of a sizeable project with a minimal dependency tree, on the other hand takes *less than 5 seconds* in the same situation.
- we get restricted to what the dependencies can let us do with their goal. The more dependencies we use, the more we'll have to rely on other developers to fix problems that affect us.
- quite often, the code of such dependencies is much more complex than it needs to be for our use-case, probably as a result of trying to be broad enough for lots of other use-cases. This makes the code less easy to understand, and also harder to debug and optimize.

On the other hand, rolling with our own code may also have its own disadvantages:
- the burden is placed on the developer to maintain code that they have to write themselves.
- there is no guarantee that the code written will turn out to be a better solution than one that is battle-tested or simply more mature.

In short, there definitely is a balance to be had when it comes to dependencies. But I believe that balance should be _heavily_ shifted to a place where 100 dependencies is extravagant or niche rather than the norm.

When it comes to native window management, the task at hand is to interact with the relevant system libraries on each operating system to spawn a window on the screen, and manage all the various properties that a window might need: resizing, fullscreen, having a title, binding to a graphics API so we can draw stuff to the screen. We shouldn't need any other dependencies than system ones, and Miniquad has shown that.

However, Miniquad is *not* a windowing project, it is a cross-platform graphics rendering project that leverages a lot of custom-made code for windowing. The one and most advanced Rust project when it comes to windowing is [Winit](https://crates.io/crates/winit). It seems to work extremely well so far and has [a lot of features under its belt](https://github.com/rust-windowing/winit/blob/master/FEATURES.md#windowing-1) for each OS they support. The problem is that it pulls out a deep nested tree of 60 dependencies for this, just on one OS. It takes several minutes to compile after a `cargo clean`. To make matters worse, if you need OpenGL, you need to add `glutin`, `glutin-winit` and `raw-window-handle` as hard dependencies too. I am convinced that we can do better than this, and have something that weighs almost nothing in terms of dependencies and compile times. That way, projects using Lokinit won't have to pull in 60 dependencies out of the box.

# Status of Lokinit

WIP

See [FEATURES.md](/FEATURES.md).

## Supported Platforms

We intend to support the 5 major native platforms, desktop and mobile:

- Desktop
  - Linux
  - MacOS
  - Windows
- Mobile
  - Android
  - iOS

We don't intend to support Web as a target. The web has vastly different needs and constraints than native systems when it comes to windowing, so if there ever is a need for it, it should probably be in a different crate with a completely different API.

(We may support Redox one day.)

# Goals of Lokinit

- **Fast compilation times.** Lokinit should compile under 5 seconds after a `cargo clean`.
- **Minimal dependencies.** We shouldn't need more than system library dependencies to do windowing properly. For such a task, 10 dependencies total is already considered too much.
- **Cross-platform.** It should support all 5 major native platforms as flawlessly as possible, potentially more OSes in the future like Redox.
- **Focus on windowing.** Our feature goals are pretty much the exact same as Winit, to spawn a window on all native platforms.
- **Easy and minimal binding to low-level graphics APIs.** People should be able to easily use Lokinit as a dependency and bind whatever graphics API they want: OpenGL, Vulkan, DirectX, Metal, or even a raw framebuffer.

# License

Lokinit is dual-licensed under APACHE and MIT.
See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
