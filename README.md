# openvr-texture-problem

Demonstrates a problem with texturing that appears using `openvr` and `glium`

## Experimental Setup

The executable opens a main window, then initializes OpenGL and OpenVR.
It then creates a program, a vertex and an index buffer for a full-screen quad, and a red-blue checker pattern texture.
In the main loop, it queries OpenVR for poses, then draws the full-screen quad to the main window.

So the main window looks like this:

![Screenshot of Correct Frame](/screenshots/correct.png?raw=true "Correct frame")

## Symptom of the Problem

To enable submitting frames to OpenVR, click anywhere on the window.

As soon as frames are submitted, the red-blue checker pattern texture becomes invalid in some way.

In subsequent frames, the main window looks like this:

![Screenshot of Incorrect Frame](/screenshots/incorrect.png?raw=true "Incorrect Frame")

## Details

- Rust toolchain version: 1.46.0
- `openvr` version: 0.6.0
- `glium` version: 0.27.0
- OpenGL vendor: NVIDIA Corporation
- OpenGL renderer: GeForce GTX 1070 Ti/PCIe/SSE2
- OpenGL version: 3.1.0 NVIDIA 432.00
- OS: Windows 10
- Headset: Oculus Rift
