local nodes = require 'daw.nodes'
local daw = require 'daw'

local graph = nodes.Graph()
graph.sample_rate = 48000
local function sawtooth(frequency)
  local constant = nodes.ConstantValue(1 / 6)
  local node = nodes.SawtoothOscillator()
  node.frequency = frequency
  local mul = nodes.Multiply()
  graph:connect(node, mul)
  graph:connect(constant, mul)
  daw.before_sample(function(sample)
    node.frequency = frequency * (sample / 48000 + 1)
  end)
  return mul
end
local function square(frequency)
  local constant = nodes.ConstantValue(1 / 16)
  local node = nodes.SquareOscillator()
  node.frequency = frequency
  local mul = nodes.Multiply()
  graph:connect(node, mul)
  graph:connect(constant, mul)
  daw.before_sample(function(sample)
    node.frequency = frequency / (sample / 48000 + 1)
  end)
  return mul
end
local add = nodes.Add()
graph:connect(sawtooth(256), add)
graph:connect(sawtooth(256 * 2 ^ (4 / 12)), add)
graph:connect(sawtooth(256 * 2 ^ (7 / 12)), add)
graph:connect(square(256), add)
graph:connect(square(256 * 2 ^ (4 / 12)), add)
graph:connect(square(256 * 2 ^ (7 / 12)), add)
graph:sink(add)

return graph
