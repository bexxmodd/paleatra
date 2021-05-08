# paleatra

Command-Line program that takes an image and produces the copy of the image
with a thin frame and palette made of the 10 most frequent colors.

# How it works

Currently, the program can be used only through the command line. 
However, the next step will be to implement a Graphical User Interface to opening image and indicating where to save

You have to clone the repo on your local machine and run cargo with two arguments:

- path to the original file.
- path where to store the copy of it.

Both paths should include file name and extension. The extension should be of image type (e.i jpg, png, etc.)

_Example_:

```bash
cargo run img/rickmorty.jpg img/results/rickmorty_cpy.jpg
```
output:

![rickmorty](https://github.com/bexxmodd/paleatra/blob/main/img/results/rickmorty_cpy.jpg?raw=true)


# TODO

- [x] Launch alpha version

- [ ] Write automated tests

- [ ] Create a GUI for the program

- [ ] Add new features

-----

## Follow Me on Social Media
<p align="center">
    <a href="https://www.twitter.com/bexxmodd">
        <img alt="twitter" src="https://i.imgur.com/fFlVB1c.png" height=40>
    </a>
    <a href="https://www.linkedin.com/in/bmodebadze">
        <img alt="linkedin" src="https://i.imgur.com/wcvwfoZ.png" height=40>
    </a>
    <a href="https://www.github.com/bexxmodd">
        <img alt="github" src="https://i.imgur.com/gnDF5oQ.png" height=40>
    </a>
</p>


Repo is distributed under the MIT license. Please see the `LICENSE` for more information.
