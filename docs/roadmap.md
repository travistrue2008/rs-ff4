- Integrate the source to `tim2` as a local project of this workspace
- Write a font loader for the text provided by the ISO

- Major goals:
  - Write a `scraper` crate that scrapes assets from the original PSP ISO, and write assets in more common formats such as `.png` for images and `.json` for data
  - Write an installer that will take the scraped assets, and convert them into assets that the rebuilt game can use
