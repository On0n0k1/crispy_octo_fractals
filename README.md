# crispy_octo_fractals
Library for generating and zooming in to customized fractals.

This is a very very VERY early state. It can be run to generate a 800 x 600 screen that shows a 
mandelbrot set. Wait for the screen to render each time before pressing the next keyboard key.
Here are the controls:

 - W A S D: Move commands to the window.

 - Q E: Zoom out, zoom in, respectively.
 
 - Esc: Close the window and the process.
 
 - Enter: Save the current image to folder "/src/saved_images/test.png".
 
If you press several keys at the same time, it will freeze running each key pressed in order. 
If you take away the focus from the screen by alt-tabbing or clicking out of the window, it 
will stop processing the events until focus is returned to the screen (I think, 
I finished this build this morning).

Here's an example of the project running:

![crispy_octo_fractals](https://github.com/On0n0k1/crispy_octo_fractals/blob/main/static/crispy_octo_fractals.png)



***So if it freezes, press ctrl-c to force the window to close.***

Here are the upcoming updates:

 - Will use async to allow the screen to move while the picture is rendering.
 
 - Will create a trait for color so that several different functions may be used for generating pictures.
It will be in directory "/src/libs/fractal/fractal_state/".
 
 - Will create a trait for processing the image that will also be in "/src/libs/fractal/fractal_state/"
 
 - Will update processing so that the image is buffered to the gpu instead of iterated by the processor.
 
## Okay, but what is all this?

Fractals are a fancy pop thing for generating pictures using a complex function. 
Each pixel in the screen is a coordinate, the algorithm iterates incrementing a 
value until it reaches a certain valid condition (or hit the max number of iterations).

The white part of the fractal are the regions where the max number of iterations has 
been reached. The dar regions are the regions where the lowest number of iterations 
were required. Every color in between is just to look pretty.

Nice fractals can be really fun to zoom into, but they also demand a lot of processing 
power to generate. So multi-threading is the only option. I'm using my library 
**kik_sync_service** for using 32 threads at the same time.

It's a great option for practicing low-level, high performance, programming. I studied 
a lot for this, but this is just scratching the surface of the iceberg. Still a long 
way ahead.

## How to install and use it:

Install rust: It's pretty fast, usually a few command lines. [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
 
**cd** into an empty folder to copy this project.
 
Copy this project with:
 
	git clone https://github.com/On0n0k1/crispy_octo_fractals.git
 
Compile the project with:
 
	cargo build
 	
This takes a while to download and install all dependencies. There will be a lot of "dead code" warnings as well.

Run the project with:
 
	cargo run
	
## Optional: Run it faster

Previous method works, but it's in debug mode. It will be faster this way:

	cargo build --release

This will compile everything into a single executable. It will be in folder "./target/release/"

	cd ./target/release/
	
	./crispy-octo-fractals
	
This runs in my linux computer. Not sure how it will look in a windows computer. 
But it will be in the same folder.

Edit: The release, optimized version still fails at saving the image. Please save the image (by pressing Enter) in debug mode only.


