# Termagotchi

**Termagotchi** is a terminal application, but it could also be your friend (I guess...). It's a very minimal Tamagotchi that runs in your terminal and has most features you would expect:

* Multiple shapes
* Growth stages
* Actions to interact with or take care of you pet.
* (short) animations
* UTC based time system, no need to keep the app running.

<br>

## How to play
There is only one goal in Termagotchi, keeping your pet alive as long as possible. All you have to do is to make sure you check on your friend regularly and give it what it needs. 

To take an action, first use the **"Up"** and **"Down"** arrow keys on your keyboard to navigate the *"Actions"* list. Then press **"Enter"** and the selected action will be performed, playing a short animation. 

When performing an action, it's corresponding "stat" is updated. Not all stats work the same but you should keep your pet's stats as high as possible. When your pet's stats get too low, it dies. This will mean you have to get a new one and start over. 

Want to exit the game? Simply press **"q"** on your keyboard. You can also just close the terminal at any moment, Termagotchi saves its state after every action you perform.

<br>

### Growing
When you first start the game you will notice that your pet will start off as an egg. It takes a couple of minutes before the egg will hatch, but then you'll have your very own Termagotchi. 

Every pet starts off looking the same, and it will take some time before you are able to see their true shape. Give it a day and you will finally see what your pet looks like.

<br>

## How to build
If you don't have the Rust tool chain installed (`rustc`, `cargo`, ... ), you should do that first at: https://www.rust-lang.org/tools/install

Next, if you don't have a C compiler installed, you will also have to do this. 
On **Windows** devices you could install *Visual Studio* and it's C/C++ dev tools to achieve this.
On **Linux** devices you can simply install *GCC* or any other C compiler you prefer. 

After doing so, you can simply clone the project, and when inside the project's directory run:
`cargo build --release`
The binary can then be found in: `{project_dir}/target/release/`

<br>

## Save file
The binary will create a save file called: `save-file.txt` in its current directory. If it ever gets moved, or the binary moves, the game will think there is no save file available and create a new one (in its current directory). 

The game is saved as plaintext in `JSON` format, so you can cheat as much as you would like. Even though I really don't know why you would cheat in a Tamagotchi game.
