#!/usr/bin/env python
from __future__ import annotations

from typing import TYPE_CHECKING
from libdaw import play
from libdaw.metronome import Metronome, TempoInstruction, Beat, BeatsPerMinute
from libdaw.nodes.envelope import Point
from libdaw.nodes import Instrument, Graph, Gain, SquareOscillator
from libdaw.notation import Sequence, loads
from libdaw.time import Time

#import copy

if TYPE_CHECKING:
    pass

sequence = loads('''+(
    %0
    *(
        +(0 1 4 2)
        =(0- 2 4),4
    )
    %4
    *(
        +(0 1 4 2)
        =(0- 2 4),4
    )
    %5
    *(
        +(0 1 4 2)
        =(0- 2 4),4
    )
    %3
    *(
        +(0 1 4 2)
        =(0- 2 4),4
    )
    %0
    *(
        +(0,0.5 2 4 2 0+ 4 2 4)
        =(0- 2 4),4
    )
    %4
    *(
        +(0,0.5 2 4 2 0+ 4 2 4)
        =(0- 2 4),4
    )
    %5
    *(
        +(0,0.5 2 4 2 0+ 4 2 4)
        =(0- 2 4),4
    )
    %3
    *(
        +(0,0.5 2 4 2 0+ 4 2 4)
        =(0- 2 4),4
    )
)''')

assert isinstance(sequence, Sequence)
    
metronome = Metronome()
metronome.add_tempo_instruction(TempoInstruction(beat=Beat(0), tempo=BeatsPerMinute(200)))

instrument = Instrument(
    factory=SquareOscillator,
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
for tone in sequence.tones(metronome=metronome):
  instrument.add_tone(tone)

graph = Graph()
gain_index = graph.add(Gain(0.15))
instrument_index = graph.add(instrument)
graph.connect(instrument_index, gain_index)
graph.output(gain_index)

play(graph)
