## Description
This is a CLI tool written in Rust.
This tool allows the user to easily change their desktop background from any location to any image from a user-defined directory.
## Usage
Add images and rename inside of the directory specified in the `$HOME/.background/conf.json` file in the `images_dir` item.
Use the following command to change the background from any location:
  ```
  background {name|arg}
  ```
- Replace `{name}` with the name of the image, with or without the file extension.
- Or replace `{arg}` with `--list` or `--ls` to get a list of files in the backgrounds directory.
- Or run with no arguments to reuse the last image set with the command.
## Installation
1. Download the background-install.sh file, either from the link below or using the following command:
    ```
    curl -LO https://github.com/DaltonJabberwo/background/releases/latest/download/background-install.sh
    ```
2. From the command line, navigate to the file's directory and run the following command:
    ```
    chmod u+x ./background-install.sh
    ```
3. Run the file:
    ```
    ./background-install.sh
    ```
## Troubleshooting
- Make sure the `/usr/bin` directory is in your PATH variable.
  - To do this, make sure the following line is at the end of your `~/.bashrc` file:
    ```
    export PATH="$PATH:/usr/bin"
    ```
