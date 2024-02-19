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
    if sample < 48000 / 4 then
      node.frequency = frequency * (sample / (48000 / 8) + 1)
    elseif sample == 48000 / 4 then
      graph:disconnect(node, mul)
      graph:disconnect(constant, mul)
    end
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
    if sample < 48000 / 4 then
      node.frequency = frequency * (sample / (48000 / 8) + 1)
    elseif sample == 48000 / 4 then
      graph:disconnect(node, mul)
      graph:disconnect(constant, mul)
    end
  end)
  return mul
end
local delay = nodes.Delay(0.5)
local gain = nodes.Multiply()
graph:connect(delay, delay)
graph:connect(delay, gain)
graph:connect(nodes.ConstantValue(0.5), gain)
graph:sink(gain)
graph:sink(nodes.ConstantValue(0))
local function echo(node)
  graph:connect(node, delay)
  graph:sink(node)
end
echo(sawtooth(256))
-- graph:connect(nodes.ConstantValue(0.75), gain)
-- graph:sink(gain)
-- local function echo(node)
--   graph:connect(node, delay)
--   graph:sink(node)
-- end
--echo()
-- echo(sawtooth(256 * 2 ^ (4 / 12)))
-- echo(sawtooth(256 * 2 ^ (7 / 12)))
-- echo(square(256))
-- echo(square(256 * 2 ^ (4 / 12)))
-- echo(square(256 * 2 ^ (7 / 12)))

return graph
