local daw = require 'daw'
local nodes = require 'daw.nodes'
local callbacks = require 'daw.callbacks'

daw.set_channels(1)
daw.set_sample_rate(48000)

local methods = {}
function methods:flip()
  self.oscillator, self.back_oscillator = self.back_oscillator, self.oscillator
end
function methods:increment()
  self.count = self.count + 1
  if self.count >= self.samples_per_switch then
    self.count = self.samples_per_switch - self.count
    self:flip()
  end
end
function methods:get_frequency()
  return self.oscillator.frequency
end
function methods:set_frequency(frequency)
  self.oscillator.frequency = frequency
  self.back_oscillator.frequency = frequency
end

local switch_metatable = {__index = {}}

function switch_metatable:__index(key)
  local method = methods[key]
  if method then
    return method
  end
  local getter = methods['get_' .. key]
  if getter then
    return getter(self)
  end
end

function switch_metatable:__newindex(key, value)
  local setter = methods['set_' .. key]
  setter(self, value)
end

function switch_metatable:__call(inputs)
  self:increment()
  return self.oscillator:process(inputs)
end

function Switch(frequency, switch_frequency)
  local sine_oscillator = nodes.SineOscillator()
  local triangle_oscillator = nodes.TriangleOscillator()
  sine_oscillator.frequency = frequency
  triangle_oscillator.frequency = frequency

  return nodes.CustomFrequency(setmetatable({
    count = 0,
    oscillator = sine_oscillator,
    back_oscillator = triangle_oscillator,
    samples_per_switch = daw.sample_rate() / switch_frequency,
    sine_oscillator = sine_oscillator,
    triangle_oscillator = triangle_oscillator,
  }, switch_metatable))
end

local instrument = nodes.Instrument(
  function()
    return Switch(256, 8)
  end,
  {
    -- start
    {
      whence = 0,
      volume = 0,
    },
    -- attack
    {
      whence = 0,
      volume = 1,
      time_offset = 0.1,
    },
    -- decay
    {
      whence = 0,
      volume = 0.6,
      time_offset = 0.2,
    },
    -- sustain
    {
      whence = 1,
      volume = 0.5,
      time_offset = -0.1,
    },
    -- zero
    {
      whence = 1,
      volume = 0,
    },
  }
)
instrument:add_note{
  start = 0,
  length = 0.5,
  frequency = 256,
}
instrument:add_note{
  start = 0.5,
  length = 0.5,
  frequency = 256 * 2 ^ (4 / 12),
}
instrument:add_note{
  start = 1,
  length = 0.5,
  frequency = 256 * 2 ^ (7 / 12),
}
return instrument

