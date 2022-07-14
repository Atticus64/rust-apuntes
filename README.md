# Apuntes de Rust 

<img src=".\assets\rustacean-flat-happy.svg" width="40%" alt="ferris-rust-mascot">

## Run app

```
cargo run
```

## Run specific script

```sh
sh run.sh src/file.rs
```

> Las notas sobre el lenguaje de Rust se encuentran en la wiki!

[Wiki](https://github.com/Tylung/rust-apuntes/wiki) 🦀

Cualquier duda o aporte a la documentación, código, etc. Pueden realizar un pull request, para agregarlo al repositorio :smiley: 

## Instalar Rust 

### Linux, Unix o Wsl

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows

1. Instalar las buildtools de Visual Studio 

[Build-Tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools)

* Ejecutar el .exe que se descarga e instalar las herramientas de Desarrollo de c++

2. Ejecutar el instalador de la pagina oficial

[Instalador-Rust](https://www.rust-lang.org/tools/install)

Si no quieres utilizar el instalador .exe, tienes 2 alternativas

**Los manejadores de paquetes de windows**:
 
Scoop

```
scoop install rustup
```

Chocolatey

```
choco install rust
```