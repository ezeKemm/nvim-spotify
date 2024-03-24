local plugin_directory = vim.fn.fnamemodify(vim.fn.resolve(vim.fn.expand("<sfile>:p")), ":h:h")

local remote_job_id = nil

local function log(msg)
	vim.api.nvim_out_write(msg .. "\n")
end

local function start_plugin()
	local bin = plugin_directory .. "/target/debug/spotify-vim"
	if not vim.fn.executable(bin) then
		log("The nvim-spotify binary is not available.")
		return
	end

	local id = vim.fn.jobstart(bin, { rpc = true })
	if id <= 0 then
		log("Failed to spawn spotify plugin")
		return
	end
	return id
end

local function like_song()
	vim.rpcnotify(remote_job_id, "like")
	vim.notify_once("rpc request to spotify : like song")
end

local function define_commands()
	vim.cmd([[command! SpotifyLike lua like_song() ]])
	vim.cmd([[command! SpfyDb lua vim.notify('Test!')]])
end

local function setup()
	-- vim.opt.completeopt = { "menuone", "noinsert", "noselect" }
	-- vim.opt.shortmess = vim.opt.shortmess + "c"

	remote_job_id = start_plugin()
	if remote_job_id ~= nil then
		-- no autocmds or refreshing? we just need to fire off a rpc request and call it a day
		define_commands()
	end
end

return {
	setup = setup,
	like_song = like_song,
}
