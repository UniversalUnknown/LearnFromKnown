# Best way to learn lanuage is by testing it 

## Table of Contents
- [Prerequirements](#requirements)
- [Installation](#installation)
- [Usage](#whats-next)
- [Pre-post Apocalyptic](#programming)

>Welcome to my Rust teaching repository! 🦀🦀🦀

I was just as new as you in rust, so let me be your companion and teach you head to toe..

Beginners requirement: ... p.p.phone maybe?

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
## Code expected?:
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

## Code: Example1:
Lets speedrun from here,
```bash
cargo new filename
```
now go to the filename folder you can read/see different directories and files in which you have **src** folder, this is where you can find **main.rs** file.
<br>
Use **nano** for linux,termux,macos. Vim is just for peoples with superiority complex so I appreciate if you learn.
<br>

For now lets use Downloads/test1 as our directory/filename
```bash
nano Downloads/test1/src/main.rs
```
now we are starting our rust code
