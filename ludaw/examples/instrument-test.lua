local daw = require 'daw'
local nodes = require 'daw.nodes'
local callbacks = require 'daw.callbacks'

daw.set_channels(2)
daw.set_sample_rate(48000)

local graph = nodes.Graph()

local instrument = nodes.Instrument(
  function()
    return nodes.SawtoothOscillator()
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
      time_offset = -1,
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
  length = 1.5,
  frequency = 256,
}
instrument:add_note{
  start = 0.5,
  length = 1.5,
  frequency = 256 * 2 ^ (4 / 12),
}
instrument:add_note{
  start = 1,
  length = 1.5,
  frequency = 256 * 2 ^ (7 / 12),
}
instrument:add_note{
  start = 2,
  length = 5,
  frequency = 256,
}
instrument:add_note{
  start = 2,
  length = 5,
  frequency = 256 * 2 ^ (4 / 12),
}
instrument:add_note{
  start = 2,
  length = 5,
  frequency = 256 * 2 ^ (7 / 12),
}
graph:output(graph:add(instrument))

local handle = callbacks.register {
  start_time = 2,
  callback = function(time)
    instrument.detune = (time - 2) / 20
  end,
}

callbacks.register {
  start_time = 7,
  callback = function()
    callbacks.cancel(handle)
  end,
  oneshot = true,
}

return graph
