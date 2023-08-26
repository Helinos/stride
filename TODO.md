## Commands
- [x] **Play**
- [ ] **Skip**: Vote skip
- [x] **Queue**: View the queue
- [ ] **Now Playing**: View the currently playing song

### DJ / Admin Only
- [x] **Forceskip**: Skip without voting
- [x] **Reorder**: Move song to be after a different song in queue
- [ ] **Remove**: Remove a song from queue
- [ ] **Leave**: Leave the voice channel and clear the queue
- [ ] **Clear**: Clear the entire queue
- [ ] **Loop**: Loop the currently playing song
- [ ] **Loopqueue**: Loop the current queue
- [ ] **Removedupes**: Remove duplicate songs
- [ ] **Absentcleanup**: Remove songs queued by people that left the voice channel
- [ ] **Favorite**: Add a song to the server's favorites
- [ ] **Playfavorite**:
 - Either add to the queue or replace it
 - Either in order or shuffled

### Admin only
- [ ] **Settings**

## Settings
- [x] **Everyone DJ**: Make everyone dj (overrides the DJ role)
- [x] **DJ Role**: Set the dj role
- [x] **DJ Only Mode**: Songs can only be played by people with the DJ role
- [x] **Announce Songs**: Bot announces each song as it comes up in the queue (Requires single channel mode)

## Fancy Commands (Commands that only work in this server)
- **Play later**:
 - Add a song to the queue at a specified time
 - If the bot isn't in a channel, join the most populated one
 - If the nobody is in the channel, wait until someone joins and then follow them in
 - Boolean if the bot should stalk users (Follow the first person to join between channels)
