# openvr-texture-problem

Demonstrates a problem with texturing that appears using `openvr` and `glium`

## Experimental Setup

The executable opens a main window, then initializes OpenGL and OpenVR.
It then creates a program, a vertex and an index buffer for a full-screen quad, and a red-blue checker pattern texture.
In the main loop, it queries OpenVR for poses, then draws the full-screen quad to the main window.

So the main window looks like this:

![Screenshot of Correct Frame](/screenshots/correct.png?raw=true "Correct frame")

## Symptom of the Problem

Now let's uncomment the block that submits frames to OpenVR:

https://github.com/mkovacs/openvr-texture-problem/blob/master/src/vr_app.rs#L87-L93

As soon as frames are submitted, the red-blue checker pattern texture seems to be corrupted somehow.

In subsequent frames, the main window looks like this:

![Screenshot of Incorrect Frame](/screenshots/incorrect.png?raw=true "Incorrect Frame")
 