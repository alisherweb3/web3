# Основы Rust 

Rust это силный и статистический язык программирования.

```Rust
let distance:f64 = Earth_Radius_In_Kilometers * central_angle;
```

```Rust
println("{:.1}", distance);
```
Что это Rust? Почему это важно?

Rust is a language that is based around safety and speed. Rust programs typically run ass fast as or faster than C++ programs. Writing concurent is trivial!


Зачем изучать Rust?


Rust memory managment is handled by Rust without the need for a garbage collector. If your code compiles, it will run without error. Native cross-platform executables. helps enforce consistency which supports governance and makes onboarding easier. Allows mentoring of developers to focus on areas other than defensive coding.


What's the Catch?

Rust has a steep learning curve. You must approach Rust programming differently.

Финальные мысли:

Rust has been the most loved language for the last several years. It's a good time to learn Rust, because big companies are investing in Rust's future.


Настройка окружения програмирования: * [Overview](#overview)



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

# Введения

### <a id="datatypes"></a>Data Types


: Инструменты разработчика

Rust Compiler is part of the Rust Toolchain.

Rust Toolchain channels: 
-> Stabel ( 6 week release cycle )
-> Beta ( 6 week release cycle )
-> Nightly ( Nightly release cycle )

Download toolchain managment utility @ https://rustup.rs

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

`rustc --version`

You need to download <a href="https://visualstudio.microsoft.com/visual-cpp-build-tools/"> Visual Studio </a>.
After this download <a href="https://www.jetbrains.com/idea/download/"> Jetbrains codespace for Rust </a>.

And u can download Visual Studio Code for programming. <a href="https://marketplace.visualstudio.com/"> Marketplace Visual Studio Code </a>

U can play with code in <a href="https://play.rust-lang.org"> Rust Sandbox </a>

## Анатомия програмирования на Rust

```Rust
#! [allow(unused_variables)]

fn main() {
  let unused_variable: u32: 0;
  println!("Hello World!:); 
}
```





## Strongly Typed Language | STRONG - 

- Animal ( Eat, Sleep )
- Duck ( Quack, Swim )
- Dog ( Runs, jumps )



--------------------------

## WEAK - more sagety than flexibility




## COMPILED AND INTERPRETER

## Stack and Heap | Data Storage Memory

## Типы Данных

Differencies in Data Types:
Numbers  |   Text Data

Scalae 


## Variables 

### CORE Design Philosophies of Rust
- Safety
- Concurrency
- Speed



## Operators


## Control Flows
- If Else Statements
- Enum and Match
- Option
- 
