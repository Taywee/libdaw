local function dump(object, indent, memo)
  indent = indent or 0
  memo = memo or {}
  local indent_str = (' '):rep(indent)
  local type_object = type(object)
  if type_object == 'table' then
    if memo[object] then
      return tostring(object)
    end
    memo[object] = true
    local inner_indent_str = (' '):rep(indent + 1)
    local lines = {'{'}
    -- First print array keys
    local len = #object
    for i=1, len do
      lines[#lines + 1] = table.concat{inner_indent_str, dump(object[i], indent + 1, memo), ','}
    end
    -- Then non-array keys
    for k, v in pairs(object) do
      if not (math.type(k) == 'integer' and k >= 1 and k <= len) then
        lines[#lines + 1] = table.concat{
          inner_indent_str,
          '[',
          dump(k, indent + 1, memo),
          '] = ',
          dump(v, indent + 1, memo),
          ','
        }
      end
    end
    lines[#lines + 1] = indent_str .. '}'
    return table.concat(lines, '\n')
  elseif object == nil or math.type(object) == 'integer' or type_object == 'boolean' or type_object == 'string' then
    return ('%q'):format(object)
  else 
    return tostring(object)
  end
end
return function(object)
  return dump(object)
end
