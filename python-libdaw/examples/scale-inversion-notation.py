#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Instrument, Graph, Gain
from libdaw.nodes.oscillators import Square
from libdaw.notation import Sequence
from libdaw.pitch import ScientificPitch
from libdaw.time import Time
#import copy

if TYPE_CHECKING:
    pass

sequence = Sequence.loads('''+(
@(g4 a b c d e f#)
*(
  +(r 1 2 3 5 4 4 6 5)
  +(r,3 1,2 7,1 1,2 2,1)
  +(=(1- 1-),3 =(5 1- 1-) =(6 6- 6-))
)
% 5
*(
  +(r 1 2 3 5 4 4 6 5)
  +(r,3 1,2 7,1 1,2 2,1)
  +(=(1- 1-),3 =(5 1- 1-) =(6 6- 6-))
)
)''')

metronome = Metronome()
metronome.add_tempo_instruction(TempoInstruction(beat=Beat(0), tempo=BeatsPerMinute(200)))
pitch_standard = ScientificPitch()

instrument = Instrument(
    factory=lambda _: Square(),
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
gain = Gain(0.075)
instrument = instrument
graph.connect(instrument, gain)
graph.output(gain)

play(graph)

