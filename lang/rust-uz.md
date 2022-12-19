# Rust Asoslari

Rust bu kuchli va statik dasturlash tili.

```Rust
let distance:f64 = Earth_Radius_In_Kilometers * central_angle;
```

```Rust
println("{:.1}", distance);
```
### Rust bu nima? Nima uchun bu muhim?


Rust tez va himoyaga asoslangan til. Rust dasturin C++ dasturidan juda tez ishlaydi. Unga qarama-qarshilik qila oladi!


### Rustni o'rganish nimaga kerak?


Rust memory managment is handled by Rust without the need for a garbage collector. If your code compiles, it will run without error. Native cross-platform executables. helps enforce consistency which supports governance and makes onboarding easier. Allows mentoring of developers to focus on areas other than defensive coding.


What's the Catch?

Rust has a steep learning curve. You must approach Rust programming differently.

### Final Fikrlar:

Rust has been the most loved language for the last several years. It's a good time to learn Rust, because big companies are investing in Rust's future.


Coding Environment Setup: * [Overview](#overview)



* [Data Types](#datatypes)
- Variables
- Operators
- Control Flow
- Ownership and Borrowing
- Functions and Error Handling
- Data Structures and Traits
- Collections
- Generics
- Concurrency[#concurrency]
- Crates and Modules[#cratesandmodules]
- Summary[#summary]

---------------------------------------


The Project

DEMO

Build our project as we cover new aspects of Rust. There will be a few self-contained bits of demo code.

DEMO: Duck Airlines.

- Create an application that will calculate the great circle route distance between two airports.
- Create an application that will calculate the distance between each waypoint along with the total distance.

----------------------------------------

# Overview 

### <a id="datatypes"></a>Data Types


: Development tools

Rust Compiler is part of the Rust Toolchain.

Rust Toolchain bo'limlari: 
- Stabel ( 6 week release cycle )
- Beta ( 6 week release cycle )
- Nightly ( Nightly release cycle )

toolchain managment utility ni mana bu sahifadan @ https://rustup.rs yuklab oling.

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

`rustc --version`

Siz <a href="https://visualstudio.microsoft.com/visual-cpp-build-tools/"> Visual Studio </a> ni yuklab olishingiz kerak.

Undan keyin esa mana buni -> <a href="https://www.jetbrains.com/idea/download/"> Jetbrains codespace for Rust </a>.

Va siz dasturlash uchun Visual Studio Code ni yuklang <a href="https://marketplace.visualstudio.com/"> Marketplace Visual Studio Code </a>

Mana bu yerda esa siz kod bilan <a href="https://play.rust-lang.org"> Rust qum qutisi </a> da o'ynashingiz mumkin.

## Rust Dasturi Ta'nasi ( Anatomiyasi)

```Rust
#! [allow(unused_variables)]

fn main() {
  let unused_variable: u32: 0;
  println!("Hello World!:); 
}
```

