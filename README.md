# Accounting GUI app

## Unfinished
An app is currently unfinished (stage 1 of 2) that means that I can use the app, but statistics are not provided yet.

## Why?
One process I wanted to improve was tracking my expenses. I used the Notes app and Excel. Each month, I had to manually calculate sums and insert them to the spreadsheet. My first attempt to save me some time was a simple python script which would calculate everything for me, although
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

### Latest update
Anybody can use app now. Assuming you know how to structure the .txt file (First line: Month Year\n Category name\n, numbers separated by \n, \n\n Next category); You have changed the hardcoded dest folder in main.rs; In that folder you have a backup folder with at least 1 excel spreadsheet (can be empty); And
probably some other assumptions, I'll make it more user friendly next year (2025).

(I'll leave this not clean tested way to build an app)
1. `git clone https://github.com/Ktoettotakoy/accounting_app`
2. `cd` to root
3. `cargo bundle --release`
4. app should be in `target/release/bundle/osx/` folder
