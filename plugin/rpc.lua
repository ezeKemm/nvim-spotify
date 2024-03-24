-- local plugin_directory = vim.fn.fnamemodify(vim.fn.resolve(vim.fn.expand("<sfile>:p")), ":h:h")
local spot = {}

spot.log = function(msg)
	vim.api.nvim_out_write(msg .. "\n")
end

local binary_path = vim.fn.fnamemodify(vim.api.nvim_get_runtime_file("plugin/rpc.lua", false)[1], ":h:h")
	.. "target/debug/spotify-vim"

spot.start = function()
	if not vim.fn.executable(binary_path) then
		spot.log("The nvim-spotify binary is not available.")
		return
	end
	print("executable found")
	spot.id = vim.fn.jobstart({ binary_path }, { rpc = true })
	if spot.id <= 0 then
		spot.log("Failed to spawn spotify plugin")
		return
	end

	print("New job...", spot.job_id)
	spot.cmds()
end

spot.notify = function(method, ...)
	spot.start()
	vim.rpcnotify(spot.id, method, ...)
	print("Sending rpc notify ...", method)
end

spot.cmds = function()
	vim.cmd([[command! SpotifyLike lua require'spot'.notify('like') ]])
	vim.cmd([[command! SpfyDb lua vim.notify('Test!')]])
end

return spot
