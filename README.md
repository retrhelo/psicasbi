# PsicaSBI
retrhelo <artyomliu@foxmail.com>

## 1. Introduction

PsicaSBI is a RISC-V SBI implementation inspired by [RustSBI](https://github.com/rustsbi/rustsbi).
This project aims for following goals:
1. Be a simple SBI implementation, both in size and coding technics.
2. An SBI implementation with clear code structure and easy to read and to modify.
3. Use Hardware Abstraction Layer (HAL) to abstract low-level hardware details.

## 2. Installation and Usage
Using PsicaSBI for your kernel is rather simple. It can be achieved by 2 steps: 

### 2.1. Install the Rust development environment. 

The official website of Rust language provides a guide on how to install a generic Rust development env. You can 
find it [here](https://www.rust-lang.org/tools/install). But by doing this you are only installing a generic and stable
env. To compile PsicaSBI, the following commands should be executed. 
```Bash
# PsicaSBI(and many other embedded rust projects) uses many features that are still unstable by now, 
# so you won't be able to compile with these features with a stable Rust. Instead use a `nightly` one. 

# Use this command to make sure that you're using nightly Rust
rustc --version

# Add RISC-V target. To be simple, we choose riscv64imac. 
rustup target add riscv64imac-unknown-elf-none
```

### 2.2. Configure PsicaSBI and compile. 

After setting up Rust, you should be able to compile this project now! But before you do so, I suggest 
that you check out some of our configs in `src/config.rs`. It contains some general configurations on 
PsicaSBI, like the size of stack and heap, the entry of your Supervisor kernel, etc. Feel free to 
change some of their values to fit your kernel better! 

And for different target platform, PsicaSBI provides features to perform _Conditional Compiling_. The 
list of features can be found in `Cargo.toml`. Check out which combination of features you'd like to 
use. 

Finally, simple use `cargo build` to compile. Or if you want to assign your own combination of features, 
use the command below. 
```Bash
cargo build --no-default-features --features="your features here"
```

## 3. Todo list
- Support embedded interrupts
- Better support for k210's SYSCTL peripheral
- See if we can introduce Device Tree for kernel
- Considering supporting more platforms?

## 4. Words to Say

I'd agree that RustSBI is an excellent SBI implementation in Rust, and that's why I "mimic" its author 
**luojia65** to write my own SBI impl in Rust instead of much-easier-to-understand C language. Before 
I started to write PsicaSBI, I was developing [xv6-k210](https://github.com/HUST-OS/xv6-k210) for an 
OS competition. At that time we were using two individual RustSBI repo called _RustSBI-QEMU_ and 
_RustSBI-k210_ for QEMU and K210 platforms. luojia65 designed his `rustsbi` as a Rust crate, and these 
two repos were instances of it. luojia65 expected that `rustsbi` to do the most common work among different
SBI implementations, and the implementing a specific repo concerns only the platform-specific codes. 
This sounds nice, but these two repos still have a large portion of same codes. They have the same boot 
codes and the same trap handler, etc. I'd say that most of codes in these two repos were the same, excepting 
those platform-related ones. Besides, these two repos are all one-file projects, which places all codes  
in `src/main.rs`, which is a waste of Rust's powerful moduling system. And of course, the codes were not 
that easy to see through. 

So I came into the idea that if I can merge these two RustSBI instances into one unified one, and use HAL 
to abstract the low-level hardware interface. I worked on that goal for a while and my work leads to 
the birth PsicaSBI. At first I planned to get my work merged into the RustSBI project. But as my work went on 
luojia65 gave out a totally-new RustSBI implementation. The new one still contains two different implementations 
for QEMU and k210, and use many technics that are either complex to understand or too language-related. So 
I finally find out that it's impossible to get my work merged into it. 

Thus I turned my work into a new project PsicaSBI. And I port it to xv6-k210 to replace the old RustSBI 
implementations. For now it works fine with xv6-k210. I must say that I used many **RustSBI's Ideas** in 
PsicaSBI, like handling the old-spec sfence.vma and rdtime instructions. And by myself I'll never find out 
how to solve these problems. But this project still have many that's different. It uses a lot of modules 
to divide different codes, and makes an advantage of Rust's "Trait" technic to abstract different peripherals. 
And what's most different, it's the deep idea behind design. 

As I'm also developing a UNIX-like kernel with my friends, I feel that the kernel's developer is very concerned 
about what SBI does. The kernel developer would like to know details about how SBI initialize a peripheral, and 
whether it can do more, or less. An SBI implementation can never satisfy the need from all different kernels, 
even if it's on the same platform. So I think it's very important to keep the SBI **clear to kernel developers**
, and make sure that kernel developers can **modify the SBI at their will**. I think that's what RustSBI failed 
to do, and what I'm working on. 

Well, after saying so many trashes (\*laugh\*), I can finally give out my key idea: Please feel free to modify 
PsicaSBI, to make it more fittable for your project. I'd better say that it is a SBI for personal modification. 

Your kernel always have more to do with the Machine Mode than limited SBI functions defined in the Spec, isn't it? 