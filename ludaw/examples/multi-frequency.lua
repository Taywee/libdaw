local daw = require 'daw'
local nodes = require 'daw.nodes'
local callbacks = require 'daw.callbacks'

daw.set_channels(2)
daw.set_sample_rate(48000)

local graph = nodes.Graph()

local detuned_square = nodes.Detune(nodes.SquareOscillator())
detuned_square.detune = 7 / 12

local frequency = nodes.MultiFrequency{
  nodes.SawtoothOscillator(),
  detuned_square,
}
frequency.frequency = 128
local gain = nodes.Gain(0.5)
local gain_index = graph:add(gain)
graph:connect(graph:add(frequency), gain_index)
graph:output(gain_index)
return graph
