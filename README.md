# Mac Events

## This app uses [Lunar](https://lunar.fyi) and [Mac Utils](https://github.com/alin23/mac-utils?tab=readme-ov-file#mirrormacbooktomonitor) to automagically switch displays when a specific device is detected.

### Why?

Back in the day, my PC and MacBook were connected to a KVM so I could switch displays and peripherals with one click. However, I had to use a separate dock on the Mac, because it lacked USB-A ports, which broke 1440p@144Hz.

Currently, my main display and dock are connected to my MacBook via separate cables, but I still wanted to be able to switch displays with one click.

This app runs in the background and watches if a specific device is connected and switches displays if it has to.

### How to get device id?

Open `System Information` -> `USB` -> find `Product ID`

### Mac Utils

Mac utils had to be patched so it doesn't mirror the display, just unmirrors it.
