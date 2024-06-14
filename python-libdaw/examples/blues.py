#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Instrument, Graph, Gain
from libdaw.nodes.oscillators import Triangle
from libdaw.notation import Sequence, loads
from libdaw.time import Time

#import copy

if TYPE_CHECKING:
    pass

sequence = loads('''+(
@(c eb f f# g bb)
1+,2 3 2 4,1 5 4 3 2 3 5
1 2
)''')
assert isinstance(sequence, Sequence)

metronome = Metronome()
for beat in range(0, 100, 2):
    metronome.add_tempo_instruction(TempoInstruction(beat=Beat(beat), tempo=BeatsPerMinute(300)))
    metronome.add_tempo_instruction(TempoInstruction(beat=Beat(beat), tempo=BeatsPerMinute(150)))
for beat in range(1, 100, 2):
    metronome.add_tempo_instruction(TempoInstruction(beat=Beat(beat), tempo=BeatsPerMinute(150)))
    metronome.add_tempo_instruction(TempoInstruction(beat=Beat(beat), tempo=BeatsPerMinute(300)))

instrument = Instrument(
    factory=lambda _: Triangle(),
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
for tone in sequence.tones(metronome=metronome):
  instrument.add_tone(tone)

graph = Graph()
gain = Gain(0.25)
graph.connect(instrument, gain)
graph.output(gain)

play(graph)

