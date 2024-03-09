local daw = require 'daw'
local nodes = require 'daw.nodes'
local callbacks = require 'daw.callbacks'

daw.set_channels(1)
daw.set_sample_rate(48000)
local oscillator_node = nodes.SineOscillator()
oscillator_node.frequency = 256

local graph = nodes.Graph()
local oscillator_index = graph:add(oscillator_node)

local custom_metatable = {}
function custom_metatable:__call(inputs)
  for _, input in ipairs(inputs) do
    for i, value in ipairs(input) do
      input[i] = value * self.gain
    end
  end
  return inputs
end
local custom_object = setmetatable({}, custom_metatable)
local custom = nodes.Custom(custom_object)

local custom_index = graph:add(custom)
graph:connect(oscillator_index, custom_index)
graph:output(custom_index)

local function callback(time)
  if custom_object.gain == 1 then
    custom_object.gain = 0.5
  else
    custom_object.gain = 1
  end
  callbacks.register{
    callback = callback,
    oneshot = true,
    start_time = 1 + time,
  }
end
callback(0)

return graph

