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

function methods:set_sharpness(sharpness)
  if sharpness < 0 or sharpness > 1 then
    error('sharpness must be between 0 and 1')
  else
    self._sharpness = sharpness
  end
end
function methods:get_sharpness()
  return self._sharpness
end

local sharp_sawtooth_metatable = {}

function sharp_sawtooth_metatable:__index(key)
  local method = methods[key]
  if method then
    return method
  end
  local getter = methods['get_' .. key]
  if getter then
    return getter(self)
  end
end

function sharp_sawtooth_metatable:__newindex(key, value)
  local setter = methods['set_' .. key]
  setter(self, value)
end

function sharp_sawtooth_metatable:__call(inputs)
  local outputs = self.oscillator:process(inputs)
  local sharpness = self.sharpness
  for _, output in ipairs(outputs) do
    for i, value in ipairs(output) do
      local negative = value < 0
      local sign
      if negative then
        sign = -1
      else
        sign = 1
      end
      value = math.abs(value)
      if value <= sharpness then
        value = 0
      else
        value = (value - sharpness) / (1 - sharpness)
      end
      output[i] = value * sign
    end
  end
  return outputs
end

-- Make a sawtooth wave with sharp points, that mostly stays flat, depending on
-- the sharpness value.
function SharpSawtooth(sharpness)
  local self = setmetatable({
    oscillator = nodes.SawtoothOscillator(),
    _sharpness = 1,
  }, sharp_sawtooth_metatable)

  self.sharpness = sharpness

  return nodes.CustomFrequency(self)
end

local instrument = nodes.Instrument(
  function()
    return SharpSawtooth(0.8)
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

