-- based on https://github.com/nvim-treesitter/nvim-treesitter/blob/master/scripts/update-readme.lua

---@class Parser
---@field name string
---@field parser ParserInfo

local os = require("os")
local ts_utils = require("nvim-treesitter.utils")

local filename = ts_utils.join_path(ts_utils.get_package_path(), "lockfile.json")
local lock = vim.fn.json_decode(vim.fn.readfile(filename))


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
    .. [[src.manual = "]] .. lock[v.name].revision .. [["]] .. "\n"
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

  local generate = ""
  if v.parser.install_info.requires_generate_from_grammar == true then
    generate = "generate = true\n"
  end

  generated_text = generated_text
    .. "[tree-sitter-" .. v.name .. "]" .. "\n"
    .. location
    .. generate
    .. "\n"
end


-- local lock = vim.fn.json_decode(vim.fn.readfile(filename))
local meta = vim.json.encode(require("nvim-treesitter.parsers").get_parser_configs())
vim.fn.writefile(vim.fn.split(meta, "\n"), "meta.json")
