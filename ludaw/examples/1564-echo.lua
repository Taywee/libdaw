local daw = require 'daw'
local nodes = require 'daw.nodes'
local callbacks = require 'daw.callbacks'

daw.set_channels(2)
daw.set_sample_rate(48000)

local graph = nodes.Graph()

local function make_echo(args)
  local delay_seconds = args.delay or error("need a delay argument")
  local gain_ratio = args.gain or error("need a gain argument")
  local graph = nodes.Graph()
  local input = graph:add(nodes.Add())
  local delay = graph:add(nodes.Delay(delay_seconds))
  local gain = graph:add(nodes.Gain(gain_ratio))
  graph:input(input)
  graph:connect(input, delay)
  graph:connect(delay, gain)
  graph:connect(gain, input)
  graph:output(gain)
  return graph
end

local echo_graph_index = graph:add(make_echo{ delay = 0.15, gain = 0.5 })
graph:output(echo_graph_index)

local function note(args)
  local octave = args.octave or 4
  local note = args.note or 0
  local node = args.node or nodes.SawtoothOscillator()
  node.frequency = 16 * 2 ^ ((octave * 12 + note) / 12)
  local delay = args.delay or 0
  local length = args.length and args.length
  local end_time = length and delay + length

  local note_graph = nodes.Graph()
  local node_index = note_graph:add(node)
  local gain_index = note_graph:add(nodes.Gain(0.1))
  note_graph:connect(node_index, gain_index)
  note_graph:output(gain_index)

  local note_graph_index

  callbacks.register{
    oneshot = true,
    start_time = delay,
    callback = function()
      note_graph_index = graph:add(note_graph)
      graph:connect(note_graph_index, echo_graph_index)
    end,
  }
  callbacks.register{
    oneshot = true,
    start_time = end_time,
    callback = function()
      graph:remove(note_graph_index)
    end,
  }
end

for i=0, 10 do 
  note{ node = nodes.SawtoothOscillator(), note = 0, delay = i * 4, length = 0.1 }
  note{ node = nodes.SawtoothOscillator(), note = 4, delay = i * 4, length = 0.1 }
  note{ node = nodes.SawtoothOscillator(), note = 7, delay = i * 4, length = 0.1 }

  note{ node = nodes.SawtoothOscillator(), octave = 3, note = 11, delay = i * 4 + 1, length = 0.1 }
  note{ node = nodes.SawtoothOscillator(), note = 2, delay = i * 4 + 1, length = 0.1 }
  note{ node = nodes.SawtoothOscillator(), note = 7, delay = i * 4 + 1, length = 0.1 }

  note{ node = nodes.SawtoothOscillator(), note = 0, delay = i * 4 + 2, length = 0.1 }
  note{ node = nodes.SawtoothOscillator(), note = 4, delay = i * 4 + 2, length = 0.1 }
  note{ node = nodes.SawtoothOscillator(), note = 9, delay = i * 4 + 2, length = 0.1 }

  note{ node = nodes.SawtoothOscillator(), note = 0, delay = i * 4 + 3, length = 0.1 }
  note{ node = nodes.SawtoothOscillator(), note = 5, delay = i * 4 + 3, length = 0.1 }
  note{ node = nodes.SawtoothOscillator(), note = 9, delay = i * 4 + 3, length = 0.1 }
end

return graph
