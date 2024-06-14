#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING, cast
from libdaw import Node, play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Add, Detune, Instrument, Graph, Gain, Implode
from libdaw.nodes.oscillators import Sawtooth
from libdaw.nodes.filters.butterworth import LowPass # noqa
from libdaw.nodes.filters import MovingAverage # noqa
from libdaw.notation import Overlapped, Sequence, loads
from libdaw.time import Duration, Time # noqa

#import copy

if TYPE_CHECKING:
    pass

def top_section_1() -> Sequence:
    lead = loads('''+>(
    d5,2 g-,1 a b c
    d,2 g- g
    e+ c,1 d e f#
    g,2 g- g
    c d,1 c b a
    b,2 c,1 b a g
    )''')
    end_a = loads('''+(
    f#,2 g,1 a b g
    a,6
    )''')
    end_b = loads('''+(
    a,2 b,1 a g f#
    g,6
    )''')
    return Sequence([lead, end_a, lead, end_b])
def bottom_section_1() -> Sequence:
    return cast(Sequence, loads('''+(
    =<(g3 b d),4 a,2
    b,6
    c
    b
    a
    g
    d+,2 b g
    d+ d-,1 c+ b a
    b,4 a,2
    g b g
    c,6
    b,2 c,1 b a g
    a,4 f#,2
    g,4 b,2
    c d d-
    g,4 g-,2
    )'''))

section_1 = Overlapped([top_section_1(), bottom_section_1()])

top_section_2 = loads('''+(
b5,2 g,1 a b g
a,2 d-,1 e f# d
g,2 e,1 f# g d
c#,2 b,1 c# a,2
a,1 b c# d e f#
g,2 f# e
f# a- c#
d,6
d,2 g-,1 f# g,2
e+ g-,1 f# g,2
d+ c b
a,1 g f# g a,2
d-,1 e f# g a b
c,2 b a
b,1 d g-,2 f#
=(g d b),6
)''')

bottom_section_2 = loads('''+(
g3,6
f#
e,2 g e
a,4 a-,2
a+,6
b,2 d c#
d f#- a
d d- c+
*(
    +(b,4 b,2)
    +(r,2 d,4)
)
*(
    +(c,4 c,2)
    +(r,2 e,4)
)
b a g
d+,4 r,2
*>(
    d-,6
    +>(r,4 f#,2)
)
e g f#
g b- d
g d g-
)''')

section_2 = Overlapped([top_section_2, bottom_section_2])

piece = Sequence([section_1, section_1, section_2, section_2])

metronome = Metronome()
metronome.add_tempo_instruction(TempoInstruction(beat=Beat(0), tempo=BeatsPerMinute(256)))

def accordian(channels: int = 1) -> Node:
    graph = Graph()
    oscillator_1 = Sawtooth()
    oscillator_2 = Sawtooth()
    oscillator_3 = Sawtooth()
    detune_2 = Detune(0.175 / 12)
    detune_3 = Detune(-0.15 / 12)
    low_pass = LowPass(order=2, frequency=1024)
    add = Add()
    implode = Implode()
    graph.input(oscillator_1)
    graph.input(detune_2)
    graph.input(detune_3)
    graph.connect(detune_2, oscillator_2)
    graph.connect(detune_3, oscillator_3)
    graph.connect(oscillator_1, add)
    graph.connect(oscillator_2, add)
    graph.connect(oscillator_3, add)
    graph.connect(add, low_pass)
    for _ in range(channels):
        graph.connect(low_pass, implode, 0)
    graph.output(implode)
    return graph

instrument = Instrument(
    factory=lambda _: accordian(),
    envelope=(
        # start
        Point(whence=0, volume=0),
        # attack
        Point(whence=0, offset=Time(0.1), volume=0.6),
        # decay
        Point(whence=0, offset=Time(0.2), volume=0.6),
        # sustain
        Point(whence=1, offset=Time(-0.05), volume=0.5),
        # zero
        Point(whence=1, volume=0),
    ),
)
for tone in piece.tones(metronome=metronome):
  instrument.add_tone(tone)

graph = Graph()
gain = Gain(0.25)
graph.connect(instrument, gain)
graph.output(gain)

play(graph)

