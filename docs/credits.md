# Credits

The following document serves as a form of bibliography to cite the sources of my knowledge and code that I might have either ported or referenced. This is a Rust learning project, so there were some opportunity to take some open source tools that were written in C, and port them to Rust. I give credit to the immediate resource that I use, although that code might have come from somewhere else (as might be mentioned in the source's README files).

## Sources

### [pspdecrypt](https://github.com/John-K/pspdecrypt)

I ported this repo to Rust (along with the `libkirk` dependency included in the repo) to decrypt the EBOOT.BIN file.

### [openkh.dev](https://openkh.dev/common/tm2.html)

This was a useful resource that I found in 2023, and was reminded of 2025 by GitHub user, _malucard_.

### [Rainbow Image Editor](https://github.com/marco-calautti/Rainbow)

I used this editor by Marco Calautti to load TIM2 files as a means of visually debugging whether my TIM2 files were decompressed correctly. I also looked at this code (among several other sources) to build a decent TIM2 image loader.

### [FF4 Tools](https://github.com/marco-calautti/FF4Tools)

This is another repo by Marco Calautti which also does a lot of the same scraping that this Rust project does. I think I found it in 2023, and started to use it as a reference to see if my code was giving similar outputs.
