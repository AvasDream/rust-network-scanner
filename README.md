[![Build Status](https://travis-ci.org/AvasDream/rust-network-scanner.svg?branch=master)](https://travis-ci.org/AvasDream/rust-network-scanner)

##  Rust Networkscanner

Networkscanner written in [Rust](https://medium.com/mozilla-tech/why-rust-is-the-most-loved-language-by-developers-666add782563)

## Prerequisites

The Networkscanner has to be run as administrator/root because it is opening raw sockets!

Under Windows you have to install [WinPcap](https://www.winpcap.org/) and place the file Packet.lib under `/target/debug/deps/Packet.lib`.
You can find the Packet.lib file in your winpcap installation here `WpdPack/Lib/x64/Packet.lib` or  here `WpdPack/Lib/Packet.lib`.

If you have not added this file you will get a linking error similiar to this
`note: Non-UTF-8 output: LINK : fatal error LNK1181: Inputfile \"Packet.lib\" could not be opened.`

Under Linux everything works fine without further steps.

## Tested with

Microsoft Windows 10 Pro 10.0.17134 Build 17134

Linux Debian Kernel 4.16.16

## Usage
```
 _____         _      _____     _                 _      _____
| __  |_ _ ___| |_   |   | |___| |_ _ _ _ ___ ___| |_   |   __|___ ___ ___ ___ ___ ___
|    -| | |_ -|  _|  | | | | -_|  _| | | | . |  _| '_|  |__   |  _| .'|   |   | -_|  _|
|__|__|___|___|_|    |_|___|___|_| |_____|___|_| |_,_|  |_____|___|__,|_|_|_|_|___|_|

Scan Types:
P          Ping scan
TF         Tcp full scan

RNS is a free Network Scanner written in rust.

Usage examples:
./rns -i 192.168.0.1 -p 79-81 -s TF
./rns -l C:\ips.txt -s P
./rns -i 192.168.0.1 -s P -o C:\out.txt

USAGE:
projekt.exe [OPTIONS] --scantype <SCANTYPE>

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-i, --ip <IP>                Set the IP to use
-l, --iplist <IP_FILE>       Set the File to read ips from
-o, --output <OUTPUT>        Set the output file
-p, --ports <PORTS>          Set the Port range to use
-s, --scantype <SCANTYPE>    Set the type of your scan
```


## License

This is free and unencumbered software released into the public domain.

Anyone is free to copy, modify, publish, use, compile, sell, or
distribute this software, either in source code form or as a compiled
binary, for any purpose, commercial or non-commercial, and by any
means.

In jurisdictions that recognize copyright laws, the author or authors
of this software dedicate any and all copyright interest in the
software to the public domain. We make this dedication for the benefit
of the public at large and to the detriment of our heirs and
successors. We intend this dedication to be an overt act of
relinquishment in perpetuity of all present and future rights to this
software under copyright law.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

For more information, please refer to <https://unlicense.org>
