local daw = require 'daw'
local pitch = require 'daw.pitch'

print(pitch.ScientificPitch():resolve{
  class = 'c',
  octave = 4,
  adjustment = 0.5,
})

print(pitch.ScientificPitch():resolve('c#[-1/2]4'))
