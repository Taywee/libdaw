local daw = require 'daw'
local nodes = require 'daw.nodes'
local callbacks = require 'daw.callbacks'

daw.set_channels(1)
daw.set_sample_rate(48000)

function Switch(frequency, switch_frequency)
  local sine_oscillator = nodes.SineOscillator()
  local triangle_oscillator = nodes.TriangleOscillator()
  sine_oscillator.frequency = frequency
  triangle_oscillator.frequency = frequency

  local custom_metatable = {__index = {}}
  function custom_metatable.__index:flip()
    self.oscillator, self.back_oscillator = self.back_oscillator, self.oscillator
  end
  function custom_metatable.__index:increment()
    self.count = self.count + 1
    if self.count >= self.samples_per_switch then
      self.count = self.samples_per_switch - self.count
      self:flip()
    end
  end
  function custom_metatable:__call(inputs)
    self:increment()
    return self.oscillator:process(inputs)
  end
  return nodes.Custom(setmetatable({
    count = 0,
    oscillator = sine_oscillator,
    back_oscillator = triangle_oscillator,
    samples_per_switch = daw.sample_rate() / switch_frequency,
    sine_oscillator = sine_oscillator,
    triangle_oscillator = triangle_oscillator,
  }, custom_metatable))
end

return Switch(256, 256)

