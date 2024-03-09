local daw = require 'daw'
local nodes = require 'daw.nodes'
local callbacks = require 'daw.callbacks'

daw.set_channels(1)
daw.set_sample_rate(48000)
local node = nodes.TriangleOscillator()
node.frequency = 48000 / 8

-- Prints:
-- 0.0
-- 0.5
-- 1.0
-- 0.5
-- 0.0
-- -0.5
-- -1
-- -0.5
-- 0.0
for _=1, 9 do
  -- First output, first channel.
  print(node:process({})[1][1])
end

return nodes.ConstantValue(0)
