local daw = require 'daw'
local nodes = require 'daw.nodes'
local callbacks = require 'daw.callbacks'

daw.set_channels(2)
daw.set_sample_rate(48000)

local metronome = daw.Metronome()
metronome:add_tempo_instruction {
  beat = 0,
  beats_per_minute = 60,
}
metronome:add_tempo_instruction {
  beat = 10,
  beats_per_minute = 128,
}
metronome:add_tempo_instruction {
  beat = 20,
  beats_per_minute = 30,
}
metronome:add_tempo_instruction {
  beat = 30,
  beats_per_minute = 500,
}

local instrument = nodes.Instrument(
  function()
    return nodes.SquareOscillator()
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
      time_offset = -0.3,
    },
    -- zero
    {
      whence = 1,
      volume = 0,
    },
  }
)

for i=0, 50 do
  local start = metronome:beat_to_time(i)
  local end_ = metronome:beat_to_time(i + 0.5)
  local length = end_ - start
  instrument:add_note{
    start = start,
    length = length,
    frequency = 256,
  }
end

return instrument
