#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import Node, play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Envelope, Instrument, Graph, Gain
from libdaw.nodes.instrument import Tone
from libdaw.nodes.oscillators import Triangle
from libdaw.notation import Item, Overlapped, Rest, Sequence
from libdaw.time import Time

#import copy

if TYPE_CHECKING:
    pass

sequence = Item.loads('''+(
@(ab4 bb c db eb f g)
1,1 1 1 2 1 1 5,2
6,1 5 6 7 1,2 1
1,1 1 1 2 1 1 5,2
6,1 5 6 7 1,2 1
5+,1 4 3 2 3 2 1,2
6,1 5 6 7 1,2 1
5,1 5 6 7 1 1 2,2
5,1 4 3 2 1,2 4 1,4
)''').element

assert isinstance(sequence, Sequence)

overlapped = Overlapped()

for offset in range(4):
    inner = Sequence(items=[
        Rest(length=Beat(offset * 8)),
        sequence,
    ])
    overlapped.append(Item(inner))
    
metronome = Metronome()
metronome.add_tempo_instruction(TempoInstruction(beat=Beat(0), tempo=BeatsPerMinute(200)))

def factory(tone: Tone) -> Node:
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
for tone in overlapped.tones(metronome=metronome):
  instrument.add_tone(tone)

graph = Graph()
gain = Gain(0.25)
graph.connect(instrument, gain)
graph.output(gain)

try:
    play(graph)
except KeyboardInterrupt:
    pass

