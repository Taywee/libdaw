#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Instrument, Graph, Gain
from libdaw.nodes.oscillators import Square
from libdaw.notation import Sequence, loads
from libdaw.pitch import ScientificPitch
from libdaw.time import Time

if TYPE_CHECKING:
    pass

sequence = loads('''+(
  1 2 3 4 5 6 7 1,2 1,1 7 6 5 4 3 2 1,2
  1,1 3 5 1 5 3 1,2
)''')
assert isinstance(sequence, Sequence)

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
gain = Gain(0.3)
graph.connect(instrument, gain)
graph.output(gain)

play(graph, channels=2, sample_rate=48000)

