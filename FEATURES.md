# Compatibility Matrix

> This matrix has been taken in large part from Winit's, readapted to show Lokinit's progress instead.

Legend:

- &check;: Works as intended
- ?: Mostly works but some bugs are known
- &cross;: Missing feature or large bugs making it unusable
- **N/A**: Not applicable for this platform

## Windowing

| Feature                          | Windows | MacOS   | Linux X11 | Linux Wayland | Android | iOS     |
| -------------------------------- | ------- | ------- | --------- | ------------- | ------- | ------- |
| Window initialization            | &check; | &check; | &check;   | &check;       | &cross; | &cross; |
| Providing pointer to init OpenGL | &cross; | &check; | &cross;   | &cross;       | &cross; | &cross; |
| Providing pointer to init Vulkan | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |
| Window decorations               | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Window decorations toggle        | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Window resizing                  | &cross; | &check; | &check;   | &cross;       | **N/A** | **N/A** |
| Window resize increments         | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Window transparency              | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Window blur                      | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Window maximization              | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Window maximization toggle       | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Window minimization              | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Fullscreen                       | &cross; | &cross; | &cross;   | &cross;       | **N/A** | &cross; |
| Fullscreen toggle                | &cross; | &cross; | &cross;   | &cross;       | **N/A** | &cross; |
| Exclusive fullscreen             | &cross; | &cross; | &cross;   | **N/A**       | &cross; | &cross; |
| HiDPI support                    | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |
| Popup windows                    | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |

## System information

| Feature          | Windows | MacOS   | Linux x11 | Linux Wayland | Android | iOS     |
| ---------------- | ------- | ------- | --------- | ------------- | ------- | ------- |
| Monitor list     | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |
| Video mode query | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |

## Input handling
| Feature                 | Windows | MacOS   | Linux x11 | Linux Wayland | Android | iOS     |
| ----------------------- | ------- | ------- | --------- | ------------- | ------- | ------- |
| Mouse events            | &cross; | &check; | &check;   | &cross;       | **N/A** | **N/A** |
| Mouse set location      | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Cursor locking          | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Cursor confining        | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Cursor icon             | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Cursor image            | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Cursor hittest          | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Touch events            | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |
| Touch pressure          | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |
| Multitouch              | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |
| Keyboard events         | &cross; | &check; | &check;   | &cross;       | &cross; | &cross; |
| Drag & Drop             | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Raw Device Events       | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |
| Gamepad/Joystick events | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |
| Device movement events  | &cross; | &cross; | &cross;   | &cross;       | &cross; | &cross; |
| Drag window with cursor | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
| Resize with cursor      | &cross; | &cross; | &cross;   | &cross;       | **N/A** | **N/A** |
