local daw = require 'daw'
local nodes = require 'daw.nodes'
local callbacks = require 'daw.callbacks'

daw.set_channels(1)
daw.set_sample_rate(48000)

local methods = {}

function methods:get_frequency()
  return self.oscillator.frequency
end
function methods:set_frequency(frequency)
  self.oscillator.frequency = frequency
end

function methods:set_flatness(flatness)
  if flatness < 0 or flatness > 1 then
    error('flatness must be between 0 and 1')
  else
    -- Might divide by zero, which will make it infinite
    self._multiplier = 1 / (1 - flatness)
  end
end
function methods:get_flatness()
  return 1 - 1 / self._multiplier
end

local flattened_sawtooth_metatable = {}

function flattened_sawtooth_metatable:__index(key)
  local method = methods[key]
  if method then
    return method
  end
  local getter = methods['get_' .. key]
  if getter then
    return getter(self)
  end
end

function flattened_sawtooth_metatable:__newindex(key, value)
  local setter = methods['set_' .. key]
  setter(self, value)
end

local min = math.min
local max = math.max
function flattened_sawtooth_metatable:__call(inputs)
  local outputs = self.oscillator:process(inputs)
  local multiplier = self._multiplier
  for _, output in ipairs(outputs) do
    for i, value in ipairs(output) do
      if value ~= 0 then
        output[i] = min(max(value * multiplier, -1), 1)
      end
    end
  end
  return outputs
end

-- Make a sawtooth wave with sharp points, that mostly stays flat, depending on
-- the flatness value.  A flatness of 1 effectively makes a square wave.
function FlattenedSawtooth(flatness)
  local self = setmetatable({
    oscillator = nodes.SawtoothOscillator(),
    _multiplier = 1,
  }, flattened_sawtooth_metatable)

  self.flatness = flatness

  return nodes.CustomFrequency(self)
end

local instrument = nodes.Instrument(
  function()
    return FlattenedSawtooth(0.95)
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
instrument:add_tone{
  start = 0,
  length = 0.5,
  frequency = 256,
}
instrument:add_tone{
  start = 0.5,
  length = 0.5,
  frequency = 256 * 2 ^ (4 / 12),
}
instrument:add_tone{
  start = 1,
  length = 0.5,
  frequency = 256 * 2 ^ (7 / 12),
}
return instrument

