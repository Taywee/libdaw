local daw = require 'daw'
local absolute = require 'daw.notation.absolute'
local nodes = require 'daw.nodes'
local pitch = require 'daw.pitch'

-- Jesu, joy of man's desiring first measure
local section = absolute.Section([=[[
  [r g4 a4 b4 d5 c5 c5 e5 d5]
  [r:3 g4:2 f#4:1 g4:2 a4:1]
  [{g2 g1}:3 {d4 g3 g1} {e4 e3 e2}]
]]=])
local metronome = daw.Metronome()
metronome:add_tempo_instruction {
  beat = 0,
  tempo = 200,
}
local pitch = pitch.ScientificPitch()

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
      offset = {
        time = 0.05,
      }
    },
    -- decay
    {
      whence = 0,
      volume = 0.6,
      offset = {
        time = 0.1,
      }
    },
    -- sustain
    {
      whence = 1,
      volume = 0.5,
      offset = {
        time = -0.01,
      }
    },
    -- zero
    {
      whence = 1,
      volume = 0,
    },
  }
)
print(daw.dump(section))
for i, tone in ipairs(section:resolve(metronome, pitch)) do
  instrument:add_tone(tone)
end
local graph = nodes.Graph()
local instrument = graph:add(instrument)
local gain = graph:add(nodes.Gain(0.3))
graph:connect(instrument, gain)
graph:output(gain)
return graph
