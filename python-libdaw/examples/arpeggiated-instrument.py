#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import Node, play
from libdaw.nodes.envelope import Point
from libdaw.nodes import Envelope, Instrument, Graph, Gain
from libdaw.nodes.instrument import Tone
from libdaw.nodes.oscillators import Triangle
from libdaw.notation import Item, Sequence
from libdaw.time import Time

#import copy

if TYPE_CHECKING:
    pass

sequence = Item.loads('''+(
1 1 1 1 <arpeggio(1/8)>=(1 3 5),4 1,1 1 1 1 <arpeggio(1/8)>=(1 2),4
)''').element

assert isinstance(sequence, Sequence)

def arpeggio(speed: float) -> float:
    return speed

def factory(tone: Tone) -> Node:
    arpeggio_speed = None
    for tag in tone.tags:
        arpeggio_speed = eval(tag)
    print(arpeggio_speed)
    graph = Graph()
    envelope = Envelope(
        length=tone.length,
        envelope=(
            # start
            Point(whence=0, volume=0),
            # attack
            Point(whence=0, offset=Time(0.1), volume=1),
            # decay
            Point(whence=0, offset=Time(0.2), volume=0.6),
            # sustain
            Point(whence=1, offset=Time(-0.05), volume=0.5),
            # zero
            Point(whence=1, volume=0),
        ),
    )
    triangle = Triangle()
    graph.connect(triangle, envelope)
    graph.input(triangle)
    graph.output(envelope)
    return graph

instrument = Instrument(factory)
for tone in sequence.tones():
  instrument.add_tone(tone)

graph = Graph()
gain = Gain(0.25)
graph.connect(instrument, gain)
graph.output(gain)

try:
    play(graph)
except KeyboardInterrupt:
    pass

