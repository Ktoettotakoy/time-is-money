# Accounting GUI app

## Unfinished
An app is currently unfinished (stage 1 of 2) that means that I can use the app, but statistics are not provided yet.

## Why?
I started learning Rust in summer 2024. As I progressed, I decided to build something high-level and useful for me.
One process I wanted to improve was tracking my expenses, which I started at the beginning of 2024. I used the Notes app and Excel. Each month, I had to manually calculate sums and insert them to the spreadsheet. My first attempt to save me some time was a simple python script which would calculate everything for me, although
I still had to manually put summaries into excel.

So I came up with a wonderful idea to create a GUI app that would save me 2-5 minutes every month. **TimeIsMoney** (the app name) should provide me with a simple way to receive and process data, then insert it into a spreadsheet. I thought that it wasn't enough, so I decided that this app should also provide me some statistics on my data stored in Excel.

### Project Outcomes
After finishing the app, I expect to:
- Finally use Rust for something (though not low-level yet)
- Read a lot of documentation
- Finally have a test coverage
- Learn more about GUI libraries for Rust
- Become a better designer
- Prove that stats modules at university are not useless
- Gain more practice with data visualization
- And the most important!!! Save time on tasks I had to do manually

#### Update (december)
I had to study a lot, now I'm back.
I still currently have to manually create a folder (hardcoded in main.rs as a const) and fill it with tmp folder, backup folder and populate backup with a lest 1 .xlsx file
I'll fix it later, but generally app should work now.
Next step is to make ui more responsive, add some ux features (like configurations for folders, and automatic creation of some files).
