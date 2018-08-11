[![Build Status](https://travis-ci.org/AvasDream/rust-network-scanner.svg?branch=master)](https://travis-ci.org/AvasDream/rust-network-scanner)

##  Rust Networkscanner

Networkscanner written in [Rust](https://medium.com/mozilla-tech/why-rust-is-the-most-loved-language-by-developers-666add782563)

## Prequisites

The Networkscanner has to run as administrator/root because it is opening raw sockets!

Under Windows you have to install [WinPcap](https://www.winpcap.org/) and place the file Packet.lib under `/target/debug/deps/Packet.lip`.
You can find the Packet.lib file in your winpcap installation here `WpdPack/Lib/x64/Packet.lib` or  here `WpdPack/Lib/Packet.lib`.
If you have not added this file you will get an linking error similiar to this
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
./rns -i 192.168.0.1 -p 0-100 -s TF
./rns -l C:\ips.txt  -p 0-100 -s P
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