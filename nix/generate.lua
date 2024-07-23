-- based on https://github.com/nvim-treesitter/nvim-treesitter/blob/master/scripts/update-readme.lua

---@class Parser
---@field name string
---@field parser ParserInfo

local parsers = require("nvim-treesitter.parsers").get_parser_configs()
local sorted_parsers = {}

for k, v in pairs(parsers) do
  table.insert(sorted_parsers, { name = k, parser = v })
end

---@param a Parser
---@param b Parser
table.sort(sorted_parsers, function(a, b)
  return a.name < b.name
end)

print(vim.inspect(sorted_parsers))

local generated_text = ""
---@param v Parser
for _, v in ipairs(sorted_parsers) do
  generated_text = generated_text
    .. "[tree-sitter-" .. v.name .. "]" .. "\n"
    .. [[src.git = "]] .. v.parser.install_info.url .. [["]] .. "\n"
    .. [[fetch.git = "]] .. v.parser.install_info.url .. [["]] .. "\n"
    .. "\n"
end

vim.fn.writefile(vim.fn.split(generated_text, "\n"), "nvfetcher.toml")

local generated_text = ""
---@param v Parser
for _, v in ipairs(sorted_parsers) do
  local location = "";
  if v.parser.install_info.location ~= nil then
    location = [[location = "]] .. v.parser.install_info.location .. [["]] .. "\n"
  end

  generated_text = generated_text
    .. "[tree-sitter-" .. v.name .. "]" .. "\n"
    .. location
    .. "\n"
end

vim.fn.writefile(vim.fn.split(generated_text, "\n"), "meta.toml")
