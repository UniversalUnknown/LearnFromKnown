# Best way to learn a lanuage is by testing it 

>Welcome to my Rust teaching repository! 🦀🦀🦀

I was just as new as you in rust, so let me be your companion and teach you 
###### :spolier not a 101 rust rather one day cletch to learn rust

Beginners requirement: ... p.p.phone maybe?

## Table of Contents
- [Prerequirements](#requirements)
- [Installation](#installation)
- [Usage](#whats-next)
- [Pre-post Apocalyptic](#programming)
- [Code Expected](#no)
<br>

## Code
- [Example 1](#example1)
- [Example 2](#example2)
- [DIY](#exercise)

## Requirements

| Your requirement | Usage required |
|--------|--------|
| mobile | [termux](https://termux.dev/en/) |
| Windows | cmd[preinstalled] |
| Mac OS | You are own ur own 💀 |
| Linux|  terminal[preinstalled]| 

## Installation
Since we are beginners lets just add better terminal for linux and MacOs, for windows try customizing your terminal with [this link](https://dev.to/ansonh/customize-beautify-your-windows-terminal-2022-edition-541l) and [ohmyposh](https://ohmyposh.dev/docs/installation/windows) cauze its cool why not..

<p align="center">
  <img src="https://github.com/UniversalUnknown/LearnFromKnown/blob/UniversalUnknown-assert-1/Main/Rust/Screenshot_20250729_213616.png" width="500" /> </p>

[CRT](https://github.com/Swordfish90/cool-retro-term) a.k.a cool retro term is a amazing terminal that look like old cathode ray tube (crt) that can elevate you love in coding 😅. If you do not like crt, still you have [ohmyzsh](https://ohmyz.sh/#install)

Lets start with installing rust <br>
<br>
- for Mobile(termux)
```yaml
pkg install rust
```
<br>

- for Linux

```yaml
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This commend helps in download for both macos and linux, in case for windows 
- for Windows
```bash
echo %PROCESSOR_ARCHITECTURE%
```
with this command one would find their architecture, now go to [rust installation](https://forge.rust-lang.org/infra/other-installation-methods.html#standalone-installers) for standalone compiler (cause windows can only download new lang along-side rather then building it within).
<br>
note* if you use zed text editor then dont worry about going throught all this, for windows vscode is way-enough... but use vscodium,blah...blah..
## What's next❓
<h4> After installing rust, you find a perfect project for yourself to fuel your thirst. </h3>

After choosing your project start to use rust with help of cargo or docker, why❔ you ask<br>
because it can **1)manage memory allocation and deallocation** rather then complicate it by allocating memory☠️ ourself(it's not cool though) well, its not end with it.<br>
<br>
**2)Who wants to document if a package manager can do that**, as you read, it can do it all by itself. <br>
<br>
**3)Unlike Asian parents it won't broke for z or A^(-1)-tier code we wrote, rather it teach how to perfect it** don't worry child-labour is not real or thats what ccp says.<br>
<br>
## Programming¿
Nope. Cargo is pending....
<br> 

run the following for linux, macos and termux,

```bash
curl https://sh.rustup.rs -sSf | sh
``` 

windows ⚰️ install .exe this definitely safe looking [file](https://win.rustup.rs/) from this definitely safe looking [website](https://doc.rust-lang.org/cargo/getting-started/installation.html)...
<br>
anyways lets start building with cargo
<br>
## No:
Create a folder in any destination you like (recommended to have easy file name rather then avg file name we create 

<p align="center">
  <img src="http://www.quickmeme.com/img/3c/3ccae4939acac4117d9b5e8cbcb52b5ce878eec293012f575ab0eac6263dff62.jpg" width="500" /> </p>

<br>

*Dont expect croped images, bruhh*
 <br>

 now try opening your terminal in the destined folder by right-click which show **open terminal** on both linux/macos and windows, whereas for termux try
<br>

```bash
mkdir name && cd name
```
this will create and move you to the created folder, if you have trust-issue(me to bro, me to) use command

```bash
pwd
```
for verification.

## Example1:
Lets speedrun from here,
```bash
cargo new filename
```
now go to the filename folder you can read/see different directories and files in which you have **src** folder, this is where you can find **main.rs** file.
<br>
Use **nano** for linux,termux,macos. Vim is just for peoples with superiority complex and I appreciate if you like it.
<br>

For now lets use Downloads/test1 as our directory/filename
```bash
nano Downloads/test1/src/main.rs
```
now we are starting our rust code
Lets make a name input and string output code
```rust
fn main() {
    //opening function
    println!("enter ur name");
    //just for user
    println!("input: ");
    //pre-input
    let mut no_one_cares = String::new();
    //assigning string to variabe, you know which one is
    std::io::stdin().read_line(&mut no_one_cares).expect("Failed");
    //rust needs you to add failsafe called .expect while readong input
    println!("You name: {no_one_cares}"); //Neo, u know it
}
```
Neo, theres also one more thing you shouldnt forgot `println!` is a [macro](https://doc.rust-lang.org/book/ch20-05-macros.html#macros) which is not essential for now. Also `//..` is just typo like `#..` .
<br>
## Example2

<br> 

- pre-requirement: Add dependency by adding `inquire = "0.7.5"` in ``Cargo.toml``, this file has been made when you created a docker file.

<br>

For example 
```tree
├── Cargo.lock
├── Cargo.toml
├── src
│  └── main.rs
└── target
    └ ..
```
        
this is the file structure, just after opening the your file right after `cargo new <filename>`. File `target` will be created after you run your program.

```toml
[package]
name = "ipop"
version = "0.1.0"
edition = "2024"

[dependencies]
inquire = "0.7.5"
```
Just add inquire version under dependence.

<br>

Since I am bored of stupid input and output, i made that no one can, nearly impossible to rebuild
...
good luck in reverse-engineering this monster called cal-seeYou-later 
```rust
use inquire::{InquireError, Select};
fn main() -> Result<(), InquireError> {
    println!("Test 2");
    //Value of A
    println!("Enter A value: ");
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).expect("Failed");
    let num1: u32 = a.trim().parse().expect("Not a valid number");
    //Value of B
    println!("Enter B vlaue: ");
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).expect("Failed");
    let num2: u32 = b.trim().parse().expect("Not a valid number");
    //Choice option
    let options = vec!["Addition", "Multiplication", "Subraction", "Division"];
    let selected_category = Select::new("Select an operation:", options).prompt()?;
    let ret: u32;
    match selected_category {
        "Addition" => ret = add(num1, num2),
        "Multiplication" => ret = mult(num1, num2),
        "Subraction" => ret = sub(num1, num2),
        "Division" => ret = div(num1, num2),
        _ => return Err(InquireError::from(Box::from("Invalid selection"))),
    }
    println!("The {selected_category} is: {ret}");
    Ok(())
}
//function
fn mult(a: u32, b: u32) -> u32 {
    return a * b;
}
fn add(a: u32, b: u32) -> u32 {
    return a + b;
}
fn sub(a: u32, b: u32) -> u32 {
    return a - b;
}
fn div(a: u32, b: u32) -> u32 {
    return a / b;
}
```
dont expect to teach me, rather copy it and paste in your main.rs file, change few variable also function name and run it.
Its not because of laziness(yes it is, i am editing for days), rather you can understand it yourself just by reading it(hopefully).
Output should look like:
[](https://github.com/UniversalUnknown/LearnFromKnown/blob/UniversalUnknown-assert-1/Main/Rust/Peek%202025-07-29%2020-55.gif)
<br>
Let me explain the backbone,
1. `InquireError`, `Select` are imported by `use` function
2. Opened a function
3. Getting value A and B as `a` and `b` variable.
4. `.trim().parse().expect("Not a valid number");` is used to convert string input to digit of value (u32)[https://docs.rs/primitive_promotion/latest/primitive_promotion/#integer-types] type under num1 or num2 variable.
5. `let options = vec!["Addition", "Multiplication", "Subraction", "Division"];`, `vec` lets you choose like frontend with moveable curor between the square bracket.
6. selected_category is.. really should i explain it neo? and `ret` is just assigning output from all those function call.
{I like to explain a lot but got pending works so 🫰}


## Exercise

I made a ~~half-crap~~ cropped exercise for user to continue the code
```rust
use inquire::{Select, InquireError};

fn get_categories() -> Vec<String> {
    vec![
        String::from("Category 1 : Create file"),
        String::from("Category 2 : Write file"),
        String::from("Category 3 : Read file"),
    ]
}

fn main() -> Result<(), InquireError> {
    let categories = get_categories();

    // Use the select function to prompt the user to choose a category
    let selected_category = Select::new("Select a category:", categories)
        .prompt()?;

    // Execute a command based on the selected category
    match selected_category.as_str() {
        "Category 1" => println!("Running command for Category 1"),
        "Category 2" => println!("Running command for Category 2"),
        "Category 3" => println!("Running command for Category 3"),
        _ => println!("No command associated with this category"),
    }

    Ok(())
}
```
yeup its based on making notes in termianl

<br>

|Fn Name |Function|
|--------|--------|
|Category 1 | Create file|
|Category 2 | Write file|
|Category 3 | Read file|

<br>

since function names are given, make functions made out of it and get setence from user and write it in file. Navigating files to store the note is simple too as i done already and a simple idea is making your own file extension that can run.
<br>

<H2>Remember Google is free to use and do not depend on any ai. Try searching in error or anthing in google so you can actually learn.</H2>
