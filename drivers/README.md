# coil-winder : drivers

For now, this is where I'll keep notes pertaining to driver software.

## Technologies

I'm going to try to use Rust. I've found a dusty old Freeduino v1.22
that I'm going to be compiling to.

![Freeduino v1.22](https://live.staticflickr.com/7058/6876947017_6297e19743.jpg)

These things were basically Arduino clones that had pre-soldered SMDs
but left the through-hole soldering up to the consumer. When I came across the
one I'm using, it was already put together for the most part. I'll stick more
information regarding the electrics in the `circuits/` folder in the root of
this repo.

## Wish List

Here are some of the things that I want to make sure I implement in the first
release of the driver.

### Preflight Checks

- Check to make sure the buck converter has a 5v output, and assist the user
  in calibration if necessary (poor man's multimeter).
- Carriage homing, using the limit switch.

### Operation

- Allow the user to specify wire gauge, core diameter, desired length, and
  perhaps the number of turns.
- Menu interface that allows for homing, multimeter stuff, setting runs, and
  run cancellation.
- Maybe preset capabilities? Or at the very least, allowing the user to rerun
  with the same options.
