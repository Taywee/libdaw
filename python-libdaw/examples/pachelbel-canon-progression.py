#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import Node, play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Envelope, Instrument, Graph, Gain
from libdaw.nodes.instrument import Tone
from libdaw.nodes.oscillators import Square
from libdaw.notation import Chord, Sequence, Mode
from libdaw.time import Time

#import copy

if TYPE_CHECKING:
    pass

chord = Chord.loads('=<(1 3 5)')
sequence = Sequence()

for progression_chord in [1, 5, 6, 3, 4, 1, 4, 5]:
    sequence.append(Mode(progression_chord))
    sequence.append(chord)

metronome = Metronome()
metronome.add_tempo_instruction(TempoInstruction(beat=Beat(0), tempo=BeatsPerMinute(60)))

def factory(tone: Tone) -> Node:
    oscillator = Square()
    envelope = Envelope(
        length = tone.length,
        envelope=(
            # start
            Point(whence=0, volume=0),
            # attack
            Point(whence=0, offset=Time(0.05), volume=1),
            # decay
            Point(whence=0, offset=Time(0.1), volume=0.6),
            # sustain
            Point(whence=1, offset=Time(-0.05), volume=0.5),
            # zero
            Point(whence=1, volume=0),
        ),
    )
    graph = Graph()
    graph.connect(oscillator, envelope)
    graph.input(oscillator)
    graph.output(envelope)
    return graph
instrument = Instrument(factory)
for tone in sequence.tones(metronome=metronome):
  instrument.add_tone(tone)

graph = Graph()
gain = Gain(0.05)
graph.connect(instrument, gain)
graph.output(gain)

play(graph)

