# Compatibility Matrix

> This matrix has been taken in large part from Winit's, readapted to show Lokinit's progress instead.

Legend:

- ✔️: Works as intended
- ❓: Mostly works but some bugs are known
- ❌: Missing feature or large bugs making it unusable
- **N/A**: Not applicable for this platform

## Windowing

| Feature                  | Windows | MacOS   | Linux X11 | Linux Wayland | Android | iOS     |
| ------------------------ | ------- | ------- | --------- | ------------- | ------- | ------- |
| Window initialization    | ❌       | ✔️       | ✔️         | ❌             | ❌       | ❌       |
| Multiple windows         | ❌       | ❌       | ✔️         | ❌             | ❌       | ❌       |
| OpenGL support           | ❌       | ❌       | ❌         | ❌             | ❌       | ❌       |
| Vulkan support           | ❌       | ❌       | ❌         | ❌             | ❌       | ❌       |
| DirectX support          | ❌       | **N/A** | **N/A**   | **N/A**       | **N/A** | **N/A** |
| Metal support            | **N/A** | ❌       | **N/A**   | **N/A**       | **N/A** | ❌       |
| Window decorations       | ❌       | ❌       | ❌         | ❌             | **N/A** | **N/A** |
| Window resizing          | ❌       | ❓       | ✔️         | ❌             | **N/A** | **N/A** |
| Window resize increments | ❌       | ❌       | ❌         | ❌             | **N/A** | **N/A** |
| Window transparency      | ❌       | ❌       | ❌         | ❌             | **N/A** | **N/A** |
| Window maximization      | ❌       | ❌       | ❌         | ❌             | **N/A** | **N/A** |
| Window minimization      | ❌       | ❌       | ❌         | ❌             | **N/A** | **N/A** |
| Fullscreen               | ❌       | ❌       | ❌         | ❌             | **N/A** | ❌       |
| Exclusive fullscreen     | ❌       | ❌       | ❌         | **N/A**       | ❌       | ❌       |
| HiDPI support            | ❌       | ❌       | ❌         | ❌             | ❌       | ❌       |
| Popup windows            | ❌       | ❌       | ❌         | ❌             | ❌       | ❌       |

## System information

| Feature          | Windows | MacOS | Linux X11 | Linux Wayland | Android | iOS |
| ---------------- | ------- | ----- | --------- | ------------- | ------- | --- |
| Monitor list     | ❌       | ❌     | ❌         | ❌             | ❌       | ❌   |
| Video mode query | ❌       | ❌     | ❌         | ❌             | ❌       | ❌   |

## Input handling

| Feature                 | Windows | MacOS | Linux X11 | Linux Wayland | Android | iOS     |
| ----------------------- | ------- | ----- | --------- | ------------- | ------- | ------- |
| Mouse events            | ❌       | ✔️     | ✔️         | ❌             | **N/A** | **N/A** |
| Mouse set location      | ❌       | ❌     | ❌         | ❌             | **N/A** | **N/A** |
| Cursor locking          | ❌       | ❌     | ❌         | ❌             | **N/A** | **N/A** |
| Cursor confining        | ❌       | ❌     | ❌         | ❌             | **N/A** | **N/A** |
| Cursor icon             | ❌       | ❌     | ❌         | ❌             | **N/A** | **N/A** |
| Cursor hittest          | ❌       | ❌     | ❌         | ❌             | **N/A** | **N/A** |
| Touch events            | ❌       | ❌     | ❌         | ❌             | ❌       | ❌       |
| Touch pressure          | ❌       | ❌     | ❌         | ❌             | ❌       | ❌       |
| Multitouch              | ❌       | ❌     | ❌         | ❌             | ❌       | ❌       |
| Keyboard events         | ❌       | ❌     | ✔️         | ❌             | ❌       | ❌       |
| Drag & Drop             | ❌       | ❌     | ❌         | ❌             | **N/A** | **N/A** |
| Raw Device Events       | ❌       | ❌     | ❌         | ❌             | ❌       | ❌       |
| Gamepad/Joystick events | ❌       | ❌     | ❌         | ❌             | ❌       | ❌       |
| Device movement events  | ❌       | ❌     | ❌         | ❌             | ❌       | ❌       |
| Drag window with cursor | ❌       | ❌     | ✔️         | ❌             | **N/A** | **N/A** |
| Resize with cursor      | ❌       | ❓     | ✔️         | ❌             | **N/A** | **N/A** |
