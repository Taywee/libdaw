for k, v in pairs({...}) do
  print(k, v)
end
local daw = require 'daw'
local graph = daw.Graph()
graph.sample_rate = 48000
local function sawtooth(frequency)
  local constant = daw.ConstantValue(1 / 6)
  local node = daw.SawtoothOscillator()
  node.frequency = frequency
  local mul = daw.Multiply()
  graph:connect(node, mul)
  graph:connect(constant, mul)
  return mul
end
local function square(frequency)
  local constant = daw.ConstantValue(1 / 16)
  local node = daw.SquareOscillator()
  node.frequency = frequency
  local mul = daw.Multiply()
  graph:connect(node, mul)
  graph:connect(constant, mul)
  return mul
end
local add = daw.Add()
graph:connect(sawtooth(256), add)
graph:connect(sawtooth(256 * 2 ^ (4 / 12)), add)
graph:connect(sawtooth(256 * 2 ^ (7 / 12)), add)
graph:connect(square(256), add)
graph:connect(square(256 * 2 ^ (4 / 12)), add)
graph:connect(square(256 * 2 ^ (7 / 12)), add)
graph:sink(add)
return graph
