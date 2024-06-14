#!/usr/bin/env python

from libdaw import play
from libdaw.nodes.envelope import Point
from libdaw.nodes.instrument import Tone
from libdaw.nodes import Instrument, Graph, Gain
from libdaw.nodes.oscillators import Square

from libdaw.time import Duration, Time, Timestamp


graph = Graph()
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
        Point(whence=1, offset=Time(-1), volume=0.5),
        # zero
        Point(whence=1, volume=0),
    ),
)


instrument.add_tone(Tone(
  start = Timestamp(0),
  length = Duration(1.5),
  frequency = 256,
))
instrument.add_tone(Tone(
  start = Timestamp(0.5),
  length = Duration(1.5),
  frequency = 256 * 2 ** (4 / 12),
))
instrument.add_tone(Tone(
  start = Timestamp(1),
  length = Duration(1.5),
  frequency = 256 * 2 ** (7 / 12),
))
instrument.add_tone(Tone(
  start = Timestamp(2),
  length = Duration(5),
  frequency = 256,
))
instrument.add_tone(Tone(
  start = Timestamp(2),
  length = Duration(5),
  frequency = 256 * 2 ** (4 / 12),
))
instrument.add_tone(Tone(
  start = Timestamp(2),
  length = Duration(5),
  frequency = 256 * 2 ** (7 / 12),
))

gain = Gain(0.1)
graph.connect(instrument, gain)
graph.output(gain)

play(graph)

# local handle = callbacks.register {
#   start_time = 2,
#   callback = function(time)
#     instrument.detune = (time - 2) / 20
#   end,
# }

# callbacks.register {
#   start_time = 7,
#   callback = function()
#     callbacks.cancel(handle)
#   end,
#   oneshot = true,
# }
