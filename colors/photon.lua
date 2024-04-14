for k in pairs(package.loaded) do
    if k:match(".*photon.*") then package.loaded[k] = nil end
end

require('photon').setup()
require('photon').colorscheme()
