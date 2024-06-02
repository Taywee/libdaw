#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import Node, play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Callback, Detune, Instrument, Graph, Gain, Multiply, TriangleOscillator
from libdaw.notation import Sequence, loads
from libdaw.time import Duration, Time, Timestamp

#import copy

if TYPE_CHECKING:
    pass

sequence = loads('''+(
e+ d#
e d# e b d c
a,3 c-,1 e a 
b,3 e-,1 g# b
c,3 e-,1 e+ d#
e d# e b d c
a,3 c-,1 e a 
b,3 e-,1 c+ b
a,6
)''')

assert isinstance(sequence, Sequence)

metronome = Metronome()
metronome.add_tempo_instruction(TempoInstruction(beat=Beat(0), tempo=BeatsPerMinute(160)))

def ilerp(a: float, b: float, c: float):
    return (b - a) / (c - a)

def triangle_bend(length: Duration) -> Node:
    length_seconds = length.seconds()
    graph = Graph()
    multiply = graph.add(Multiply())
    detune = Detune(-1 / 12)
    detune_index = graph.add(detune)
    graph.input(multiply)
    graph.connect(detune_index, multiply)
    triangle = graph.add(TriangleOscillator())
    graph.connect(multiply, triangle)
    graph.output(triangle)
    def _callback(timestamp: Timestamp) -> None:
        timestamp_seconds = timestamp.seconds()
        interpolation = ilerp(0.0, timestamp_seconds, length_seconds)
        detune.detune = (1.0 - interpolation) * (-1 / 6) 
    callback = Callback(graph)
    callback.add(_callback, end=Timestamp(length_seconds))
    return callback

instrument = Instrument(
    factory=triangle_bend,
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

