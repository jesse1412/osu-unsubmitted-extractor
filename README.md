# osu-unsubmitted-extractor
Extracts all unsubmitted songs from a legacy osu! songs folder.

Kurboh requested all of my unsubmitted beatmaps. I couldn't see an easy way to get them, so I built a small tool to facilitate their extraction.

# Running (windows only, linux/other will need to build manually)
1. [Create a new OAuth application](https://osu.ppy.sh/home/account/edit#oauth) and note down the client ID and secret key.
2. Download the latest binary file from [the releases page](https://github.com/jesse1412/osu-unsubmitted-extractor/releases)
3. Open a terminal and navigate to the folder where the binary is saved.
4. Run the binary `osu-unsubmitted-extractor.exe -s C:\your\path\to\osu!\Songs --secret  2IMNDdDLWt92MXtnbNpXGugrPNxBBcn3yKeJdts5 -c 37692 -o songs.tar`

If you're stuck, chatGPT will understand.

# Building manually
1. [Create a new OAuth application](https://osu.ppy.sh/home/account/edit#oauth) and note down the client ID and secret key.
2. Install the rust compiler (recommended via [rustup](https://www.rust-lang.org/tools/install))
3. Clone this repository `git clone https://github.com/jesse1412/osu-unsubmitted-extractor.git`
4. Open a CLI in the cloned repo/folder.
5. Run the build with your client ID and secret: `cargo run --release -- -s C:\your\path\to\osu!\Songs --secret  2IMNDdDLWt92MXtnbNpXGugrPNxBBcn3yKeJdts5 -c 37692 -o songs.tar`.
6. If you want to know more about the params, run `cargo run --release -- --help`.
7. Your unsubmitted beatmaps will be packaged into the .tar file specified by the -o argument (open it with 7zip, etc).

# Example run time details (my personal run)
```
[2025-01-25T12:45:41Z INFO  osu_unsubmitted_extractor] Run complete
[2025-01-25T12:45:41Z INFO  osu_unsubmitted_extractor] Processed 11961 song folders
[2025-01-25T12:45:41Z INFO  osu_unsubmitted_extractor] Scanned 127820 files in song dirs
[2025-01-25T12:45:41Z INFO  osu_unsubmitted_extractor] Processed 39294 .osu files
[2025-01-25T12:45:41Z INFO  osu_unsubmitted_extractor] Pulled API details for 12667 .osu files
[2025-01-25T12:45:41Z INFO  osu_unsubmitted_extractor] Made 254 API calls
[2025-01-25T12:45:41Z INFO  osu_unsubmitted_extractor] Tarred 687 song folders
[2025-01-25T12:45:41Z INFO  osu_unsubmitted_extractor] Total runtime: 752s
```

Tar size: 5.82GB.
