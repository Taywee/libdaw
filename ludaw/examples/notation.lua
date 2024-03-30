local daw = require 'daw'
local absolute = require 'daw.notation.absolute'
local nodes = require 'daw.nodes'
local pitch = require 'daw.pitch'

local function dump(o, indent)
  indent = indent or 0
  local indent_str = (' '):rep(indent)
  local type_o = type(o)
  if type_o == 'table' then
    local inner_indent_str = (' '):rep(indent + 1)
    local lines = {'{'}
    -- First print array keys
    local len = #o
    for i=1, len do
      lines[#lines + 1] = table.concat{inner_indent_str, dump(o[i], indent + 1), ','}
    end
    -- Then non-array keys
    for k, v in pairs(o) do
      if not (math.type(k) == 'integer' and k >= 1 and k <= len) then
        lines[#lines + 1] = table.concat{
          inner_indent_str,
          '[',
          dump(k, indent + 1),
          '] = ',
          dump(v, indent + 1),
          ','
        }
      end
    end
    lines[#lines + 1] = indent_str .. '}'
    return table.concat(lines, '\n')
  elseif o == nil or math.type(o) == 'integer' or type_o == 'boolean' or type_o == 'string' then
    return ('%q'):format(o)
  else 
    return tostring(o)
  end
end


-- Jesu, joy of man's desiring first measure
local section = absolute.Section([[{
  {r:1 g4:1 a4:1 b4:1 d5:1 c5:1 c5:1 e5:1 d5:1}
  {r:3 g4:2 f#4:1 g4:2 a4:1}
  {r:3 d4:3 e4:3}
}]])
local metronome = daw.Metronome()
local pitch = pitch.ScientificPitch()

local instrument = nodes.Instrument(
  function()
    return nodes.SineOscillator()
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
for i, tone in ipairs(absolute.resolve_section(section, metronome, pitch)) do
  instrument:add_tone(tone)
end
local graph = nodes.Graph()
local instrument = graph:add(instrument)
local gain = graph:add(nodes.Gain(0.3))
graph:connect(instrument, gain)
graph:output(gain)
return graph
