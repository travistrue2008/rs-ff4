- Integrate the source to `vex` as a local project of this workspace
  - Update `vex` to work with the latest rust toolchain
  - Remove raw memory layout of vector/matrix
- Integrate the source to `tim2` as a local project of this workspace
- Replace `gl_toolkit` with `wgpu` crate
- Write a font loader for the text provided by the ISO
- Deprecate the `scraper_bestiary` crate, and scrape enemy/boss data from the ISO instead
- `unpacker` should also unpack files from the ISO

- Major goals:
  - Write an unpacker crate that will unpack the large `.dat` files from the PSP ISO into a directory structure
  - Write a `scraper` crate that scrapes assets from the original PSP ISO, and write assets in more common formats such as `.png` for images and `.json` for data
  - Write an installer that will take the scraped assets, and convert them into assets that the rebuilt game can use