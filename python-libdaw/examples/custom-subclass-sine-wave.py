#!/usr/bin/env python

from __future__ import annotations

from collections.abc import Sequence
from libdaw import play, Sample
from libdaw.nodes import Custom, Graph, Gain, ConstantValue
from math import tau, sin

class CustomSineOscillator(Custom):
    def __new__(cls: type[CustomSineOscillator], *args, **kwargs):
        return Custom.__new__(cls)

    def __init__(self, frequency: float = 256, channels: int = 2, sample_rate: int = 48000):
        self.callable = self
        self.__channels = channels
        self.__frequency = frequency
        self.__sample_rate = sample_rate
        self.__ramp = 0

    def __call__(self, inputs: Sequence[Sample]) -> Sequence[Sample]:
        try:
            frequency = inputs[0][0]
        except IndexError:
            frequency = self.__frequency
        ramp_delta = frequency * tau / self.__sample_rate
        value = sin(self.__ramp)
        self.__ramp += ramp_delta
        return (Sample([value] * self.__channels),)


graph = Graph()
constant = graph.add(ConstantValue(440))
custom = graph.add(CustomSineOscillator())
gain = graph.add(Gain(0.5))
graph.connect(constant, custom)
graph.connect(custom, gain)
graph.output(gain)

play(graph)

