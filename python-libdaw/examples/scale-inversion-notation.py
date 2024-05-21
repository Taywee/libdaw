#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Instrument, Graph, Gain, SquareOscillator
from libdaw.notation import Sequence
from libdaw.pitch import ScientificPitch
from libdaw.time import Time
#import copy

if TYPE_CHECKING:
    pass

sequence = Sequence.loads('''+(
@(g4 a b c d e f#)
*(
  +(r 0 1 2 4 3 3 5 4)
  +(r,3 0,2 6,1 0,2 1,1)
  +(=(0- 0-),3 =(4 0- 0-) =(5 5- 5-))
)
% 5
*(
  +(r 0 1 2 4 3 3 5 4)
  +(r,3 0,2 6,1 0,2 1,1)
  +(=(0- 0-),3 =(4 0- 0-) =(5 5- 5-))
)
)''')

# minor_sequence = copy.deepcopy(sequence)
# # Take the scale and make it minor by shifting two to the right
# minor_sequence[0].insert(0, minor_sequence[0].pop())
# minor_sequence[0].insert(0, minor_sequence[0].pop())
# sequence.append(minor_sequence)

metronome = Metronome()
metronome.add_tempo_instruction(TempoInstruction(beat=Beat(0), tempo=BeatsPerMinute(200)))
pitch_standard = ScientificPitch()

instrument = Instrument(
    factory=SquareOscillator,
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
for tone in sequence.tones(metronome=metronome, pitch_standard=pitch_standard):
  instrument.add_tone(tone)

graph = Graph()
gain_index = graph.add(Gain(0.3))
instrument_index = graph.add(instrument)
graph.connect(instrument_index, gain_index)
graph.output(gain_index)

play(graph)
