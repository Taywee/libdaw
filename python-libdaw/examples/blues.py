#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Instrument, Graph, Gain, TriangleOscillator
from libdaw.notation import Sequence, loads
from libdaw.time import Time

#import copy

if TYPE_CHECKING:
    pass

sequence = loads('''+(
@(c eb f f# g bb)
0+,2 2 1 3,1 5 4 3 2 1 2
0 1
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
for tone in sequence.tones(metronome=metronome):
  instrument.add_tone(tone)

graph = Graph()
gain_index = graph.add(Gain(0.25))
instrument_index = graph.add(instrument)
graph.connect(instrument_index, gain_index)
graph.output(gain_index)

play(graph)

