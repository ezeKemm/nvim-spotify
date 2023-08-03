" Initialize the channel for nvim-spotify
if !exists('s:spotifyjobid')
	let s:spotifyjobid = 0
endif

" Path to the binary
let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = s:scriptdir . '/target/release/nvim-spotify'

" RPC message constants
let s:SpotifyLike = 'like'
let s:SpotifyUnlike = 'unlike'
" let s:Spotifycurrentsong = 'spotify_current_song'
" let s:spotifyplaypause = 'spotify_play_pause'
" let s:spotifyplay = 'spotify_play'
" let s:spotifypause = 'spotify_pause'
" let s:spotifynext = 'spotify_next'
" let s:spotifyprevious = 'spotify_previous'
" let s:spotifylyrics = 'spotify_lyrics'

" entry point
function! s:init()
  call s:connect()
endfunction

" get the job id and check for errors. if no errors, attach rpc handlers to
" the commands.
function! s:connect()
  let jobID = s:GetJobID()

  if 0 == jobID
    echoerr "spotify: cannot start rpc process"
  elseif -1 == jobID
    echoerr "spotify: rpc process is not executable"
  else
    let s:spotifyjobid = jobID
    call s:AttachRPCHandlers(jobID)
  endif
endfunction

" Function reference in case of RPC start errors
function! s:OnStderr(id, data, event) dict
  echom 'stderr: ' . a:event . join(a:data, "\n") 
endfunction

" Start the RPC job and return the job  (channel) ID
function! s:GetJobID()
  if s:spotifyjobid == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true, 'on_stderr': function('s:OnStderr') })
    return jobid
  else
    return s:spotifyjobid
  endif
endfunction

" Associate commands with their RPC invocations
function! s:AttachRPCHandlers(jobID)
  command! -nargs=0 SpotifyLike :call s:rpc(s:SpotifyLike)
  command! -nargs=0 SpotifyUnlike :call s:rpc(s:SpotifyUnlike)
  "
  " command! -nargs=0 SpotifyCurrentSong :call s:rpc(s:SpotifyCurrentSong)
  " command! -nargs=0 SpotifyPlayPause :call s:rpc(s:SpotifyPlayPause)
  " command! -nargs=0 SpotifyPlay :call s:rpc(s:SpotifyPlay)
  " command! -nargs=0 SpotifyPause :call s:rpc(s:SpotifyPause)
  " command! -nargs=0 SpotifyNext :call s:rpc(s:SpotifyNext)
  " command! -nargs=0 SpotifyPrevious :call s:rpc(s:SpotifyPrevious)
  " command! -nargs=0 SpotifyLyrics :call s:rpc(s:SpotifyLyrics)
endfunction

" Send an RPC message to the remote process.
function! s:rpc(rpcMessage)
	call rpcnotify(s:spotifyjobid, a:rpcMessage)
endfunction

call s:init()
