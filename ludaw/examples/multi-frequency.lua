local daw = require 'daw'
local nodes = require 'daw.nodes'
local callbacks = require 'daw.callbacks'

daw.set_channels(2)
daw.set_sample_rate(48000)

local graph = nodes.Graph()

local detunes = {}
detunes[1] = nodes.Detune(nodes.SineOscillator())
detunes[2] = nodes.Detune(nodes.SineOscillator())

local just
local function switch_intonation(time)
  time = time or 0
  just = not just
  if just then
    detunes[1].detune = math.log(5 / 4) / math.log(2)
    detunes[2].detune = math.log(6 / 4) / math.log(2)
  else
    detunes[1].detune = 4 / 12
    detunes[2].detune = 7 / 12
  end

  callbacks.register {
    start_time = 2 + time,
    oneshot = true,
    callback = switch_intonation,
  }
end
switch_intonation(0)

local frequency = nodes.MultiFrequency{
  nodes.SineOscillator(),
  table.unpack(detunes),
}
frequency.frequency = 256
local gain = nodes.Gain(0.3)
local gain_index = graph:add(gain)
graph:connect(graph:add(frequency), gain_index)
graph:output(gain_index)

return graph
