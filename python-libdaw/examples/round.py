#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Instrument, Graph, Gain, TriangleOscillator
from libdaw.notation import Overlapped, Rest, Sequence, loads
from libdaw.time import Time

#import copy

if TYPE_CHECKING:
    pass

sequence = loads('''+(
@(ab4 bb c db eb f g)
0,1 0 0 1 0 0 4,2
5,1 4 5 6 0,2 0
0,1 0 0 1 0 0 4,2
5,1 4 5 6 0,2 0
4+,1 3 2 1 2 1 0,2
5,1 4 5 6 0,2 0
4,1 4 5 6 0 0 1,2
4,1 3 2 1 0,2 3 0,4
)''')

overlapped = Overlapped()

for offset in range(4):
    inner = Sequence(items=[
        Rest(length=Beat(offset * 8)),
        sequence
    ])
    overlapped.append(inner)
    
metronome = Metronome()
metronome.add_tempo_instruction(TempoInstruction(beat=Beat(0), tempo=BeatsPerMinute(200)))

instrument = Instrument(
    factory=TriangleOscillator,
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
for tone in overlapped.tones(metronome=metronome):
  instrument.add_tone(tone)

graph = Graph()
gain_index = graph.add(Gain(0.25))
instrument_index = graph.add(instrument)
graph.connect(instrument_index, gain_index)
graph.output(gain_index)

play(graph)
