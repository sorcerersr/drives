# drives

A rust library (crate) for listing mounted or mountable drives on linux (flash drives, sd-cards, etc.)

Uses the virtual sysfs filesystem (/sys) to gather information about the block devices known by the linux kernel.

## Changelog

### v.0.0.1

initial prototype

### v.0.0.2

second prototype with a little bit more information gathered from /sys/block - but still in a very early stage - work in progress