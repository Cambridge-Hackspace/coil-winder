# coil-winder : electrics

This is where I'm going to put stuff regarding circuits and other electronic
components.

## Technologies

We're in prototyping mode! So I'm abusing protoboards like nobody's business.
Here are the parts I'm working with:

- 1x Freeduino v1.22
- 1x 1602A LCD Screen
- 1x I2C LCD Backpack w/ PCF8574 and Contrast Potentiometer
- 1x Analog Joystick Module
- 1x Limit Switch
- 1x Buck Converter
- ?x Resistors (see below)
- 2x 28BYJ-48 Stepper Motors
- 2x ULN2003 Stepper Driver Boards

## Pin Limitations

The first thing that I dealt with was the pin shortage. The Freeduino board I'm
using has the following:

- 1x RST pin
- 1x 3.3v pin
- 1x 5v pin
- 3x GND pins
- 1x VIN pin
- 1x AREF pin
- 6x analog pins
- 1x digital tx pin
- 1x digital rx pin
- 6x digital PWM pins
- 6x digital generic pins

So immediately I have a problem in terms of pin count. I essentially have four
(4) main "domains": (a) the motors, (b) the limit switch, (c) the LCD, and (d)
the joystick module. Without assistance, that would normally require a total of
twenty-one (21) pins: four (4) for each motor, one (1) for the limit switch,
six (6) for the LCD, and seven (7) for the joystick (for a silly reason, which
I'll go into more depth on). I want to prioritize the motors and limit switch--
I didn't want to cause any undue lag in either of those domains, so I've put in
some additional components to squeeze the LCD and joystick into a fewer number
of pins on the Freeduino itself.

### Joystick

I found some really cheap joysticks, and you get what you pay for. For my
purposes, they'll do perfectly, but the pinout is kinda silly.

![KOOBOOK Joystick](https://images-na.ssl-images-amazon.com/images/I/71ffhlbKhHL._AC_UL495_SR435,495_.jpg)

These boards don't even have a serial number on them. It's essentially a
joystick that sits atop five tactile switches--each of which are either on
or off. You have up, down, left, and right directional buttons, but you can
also press down on the joystick for a "fifth" direction. There's also SET and
RST tactile switches. Each switch closes an individual pin. Dead simple. I
probably could have just soldered some tactile switches floating around but
quite frankly Max brought some joystick modules one day and I got fomo. Didn't
realize until it was too late that the inputs didn't float. Oops.

Okay, so what do we do about this? Well, I opted to build a resistor ladder.
Essentially, instead of treating each of the pins as a digial output, we can
choose various resistors such that each combination of button presses yields
a unique voltage drop. We can read that on an analog pin and reconstruct
exactly which buttons were pressed.

So that's all well and good, but with any design you'll see here there's a
modicum of planning, however chaotic. And in this case there were a few things
I wanted to keep in mind.

- I wanted to make sure I cared about possible button combinations, but not
  care about _impossible_ button combinations. So for example, the directional
  UP and RIGHT buttons can be pressed at the same time (e.g. moving northeast)
  but the directional UP and DOWN buttons cannot physically be pressed at the
  same time--so I don't want to waste compute cycles on it (and I want to give
  myself more options for viable resistor choices).
- Each button combination yields a particular voltage drop. I wanted to make
  sure that the smallest delta among the various neighboring voltage drop
  expectations was large enough that I could viably and definitively measure
  it in the driver--I don't want the driver to have to guess or use any more
  heuristic mechanisms that are absolutely required. In other words, I want
  to make sure that the various expected voltage drops are both vaguely
  equidistant and vaguely distributed.
- I wanted to use common resistors. Not that we have a shortage. But I'm too
  lazy to chain resistors.

I actually ran into a bit of a problem with the second requirement here, as
with seven buttons I'd have $2^{7}=128$ possible combinations. I mean, I could
disregard those impossible combinations (see the first requirement), but in
that case I really only save $\sum_{k=1}^{2}(-1)^{k-1}\binom{2}{k}2^{x-2k}=56`,
leaving $128-56=72$ combinations... and we'd start running into problems
pinpointing the proper "window" when measuring the voltage drop to determine
which buttons were actually pressed. So instead of building one ladder, I built
two.

I came up with following set of resistors:

- 1x 10k
- 2x 4k7
- 2x 2k2
- 4x 1k

And I separated the "directional" buttons from the "action" buttons. (Even
though the CENTER button ("MID" on the board) was physically on the joystick,
I felt that it made more sense from the perspective of the user to treat it
as an action--folks are really only going to be hitting that middle button
to confirm a directive. That change, paired with the "forbidden" pairs, made
sure that the ladders weren't too lopsided: we end up with a mere nine (9)
combinations over on the directional ladder, and eight (8) combinations over
on the action ladder.

Here's the circuit I ended up building:

```

                       ┌────┐  ┌─────┐  ┌────┐                        
┌──────────────────────┤ A0 │  │ GND │  │ A1 ├───────────────────────┐
│                      └────┘  └──┬──┘  └────┘                       │
│                                 │                                  │
│  ┌───────┐  ┌─────┐             │                                  │
├──┤ UP    ├──┤ 4k7 ├──┐          │                                  │
│  └───────┘  └─────┘  │          │             ┌─────┐  ┌────────┐  │
│                      │          │          ┌──│ 1k  ├──┤ CENTER ├──┤
│  ┌───────┐  ┌─────┐  │          │          │  └─────┘  └────────┘  │
├──┤ DOWN  ├──┤ 4k7 ├──┤          │          │                       │
│  └───────┘  └─────┘  │  ┌────┐  │  ┌────┐  │  ┌─────┐  ┌────────┐  │
│                      ├──┤ 1k ├──┴──┤ 1k ├──┼──┤ 2k2 ├──┤ SET    ├──┤
│  ┌───────┐  ┌─────┐  │  └────┘     └────┘  │  └─────┘  └────────┘  │
├──┤ LEFT  ├──┤ 2k2 ├──┤                     │                       │
│  └───────┘  └─────┘  │                     │  ┌─────┐  ┌────────┐  │
│                      │                     └──│ 4k7 ├──┤ RESET  ├──┘
│  ┌───────┐  ┌─────┐  │                        └─────┘  └────────┘   
└──┤ RIGHT ├──┤ 1k  ├──┘                                              
   └───────┘  └─────┘                                                 

```

I'll upload pictures of my shoddy soldering when I remember next. The code that
actually interprets the ladder will be up in the `drivers/` readme when I get
around to it as well.

### LCD

So this one was a lot easier, mostly because LCD backpacks are pretty cheap and
I wasn't about to try to build my own. So normally, the
[1602A](https://mm.digikey.com/Volume0/opasdata/d220001/medias/docus/5773/CN0295D%20other%20related%20document.pdf)
expects a parallel data pipeline across six (6) digital pins, and the I2C
backpack essentially allows us to send serial data across two (2) analog pins
instead. We specifically need the SDA (serial data) and SCL (serial clock)
pins, corresponding to A4 and A5 on the Freeduino.

## Power Limitations

Supposedly, the motors themselves (along with the other components on the
microcontroller) should pull less than 1A.

We're using two [28BYJ-48](https://www.mouser.com/datasheet/2/758/stepd-01-data-sheet-1143075.pdf)
stepper motors here, which should have a DC resistance of
$50\Omega \pm 7%\geq 46.5\Omega$. We'll assume that we may be energizing two
coils at a time and of course we have two motors, so the motors could pull
just over $\frac{5\,\mathrm{V}}{46.5\,\Omega} \approx 107.5\,\mathrm{mA}$
per coil. (Though I should note that real-world testing seems to indicate
that a single coil could peak as high as 135 mA). That by itself isn't a
problem in terms of current until you consider a potential thermal shutdown
event as the onboard regulator tries to step 12V down to 5V over 540 mA
(so, two motors with two coils at peak -- and this is without any consideration
of the current by the LCD, backpack, or the Freeduino itself). We could expect
the linear regulator on the Freeduino to try to burn off
$(12\,\mathrm{V}-5\,\mathrm{V})*0.54\,\mathrm{A}=3.78\,\mathrm{W}$, bringing us
well over the $150\,\degree\mathrm{C}$ limit (assuming $50\,\degree\mathrm{C}$
per watt).

So, we're using a buck converter. Assuming that the buck converter itself is
vaguely (in)efficient--let's say, perhaps 80%--we can posit that the upstream
side will probably cap out at maybe
$\frac{3.78\,\mathrm{W}}{12\,\mathrm{V}}*\frac{1}{0.8}=393.75\,\mathrm{mA}$,
which should be the largest load on the Freeduino (and well below the 1000mA
maximum enforced by the upstream protection diode).

I've kinda gone back and forward between three options: (a) I could connect
the buck converter to the VIN pin and take advantage of the 12V potential
available there; (b) I could splice into the 12V adapter's line (or get a
different barrel jack); or (c) I could solder directly to the existing barrel
jack pins, before the diode. Option (a) has the downside of being behind the
protection diode, so I'm gambling that we don't have a current spike. Option
(b) requires direct soldering to the Freeduino--and I've been trying to keep
it in a state where it can be easily swapped out if necessary. And option (c),
well, I don't wanna. (I have a perfectly good barrel plug already conveniently
on the board, and I don't really want to treat the motors like a separate
device--I'd like this thing to be one cohesive unit.) So I think I'm heavily
leaning toward option (a), which allows me to connect the buck converter to
the protoboard I've been working with already.

Now, the buck converter I'm using has an easy-to-adjust variable voltage screw,
which is a double-edged sword. On one hand, it's super easy to access, but on
the other hand, vibrations will probably bump it around and it may need future
calibration. So one of the things I'm going to want to do is to use the
remaining analog pins (A2 and A3) to act as a sort of voltmeter, and to
periodically run a safety check on the Freeduino itself to make sure we're not
overvolting the steppers. When I get around to doing that (and I figure it'll
likely be one of the last things I do, as far as prototyping the electrical
system is concerned), I'll need to remember to cut the potential through
some resistors--don't want to go through all that effort of tying in a buck
converter, just to fry the board in the event we jump over 5V. (Though, I
suppose magic smoke is perhaps one of the most obvious voltmeters out there...)
