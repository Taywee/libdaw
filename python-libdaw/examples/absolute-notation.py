#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Instrument, Graph, Gain, SquareOscillator
from libdaw.notation import Overlapped, Sequence, parse, Chord, Note
from libdaw.pitch import ScientificPitch
from libdaw.time import Time
from functools import partial

if TYPE_CHECKING:
    from libdaw.notation import _Item

sequence = parse('''+(
*(
  +(r g4 a b d c c e d)
  +(r:3 g4:2 f#:1 g:2 a:1)
  +(=(g3 g-1):3 =(d g-1 g-1) =(e e-1 e-1))
)
)''')
assert isinstance(sequence, Sequence)

def change_adjustments(item: _Item, amount: float = 0):
    match item:
        case Overlapped():
            for subitem in item:
                change_adjustments(subitem, amount)
        case Sequence():
            for subitem in item:
                change_adjustments(subitem, amount)
        case Chord():
            for pitch in item:
                pitch.pitch_class.adjustment += amount
        case Note():
            item.pitch.pitch_class.adjustment += amount

change_adjustments(sequence, 0)

metronome = Metronome()
metronome.add_tempo_instruction(TempoInstruction(beat=Beat(0), tempo=BeatsPerMinute(200)))
pitch_standard = ScientificPitch()

instrument = Instrument(
    factory=partial(SquareOscillator, channels=2, sample_rate=48000),
    sample_rate=48000,
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

play(graph, channels=2, sample_rate=48000)

