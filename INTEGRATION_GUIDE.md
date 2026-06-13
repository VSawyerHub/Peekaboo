# Platform Integration Guide

## Twitch Integration ✅ (Fully Implemented)

### Setup:
1. Get OAuth token from [twitchapps.com/tmi](https://twitchapps.com/tmi/)
2. Requires `chat:read` scope
3. Enter channel name and token in dashboard
4. Triggers:
   - **Bits**: Configurable threshold (default 100)
   - **Subs/Resubs/Gift Subs**: Always triggers jumpscare

---

## YouTube Integration 🚧 (Stub - Needs Implementation)

### What's needed:
1. **YouTube Data API v3** access from Google Cloud Console
2. **Live Chat API** polling implementation
3. OAuth 2.0 flow for user authentication

### Setup (when implemented):
1. Create project in [Google Cloud Console](https://console.cloud.google.com/)
2. Enable YouTube Data API v3
3. Create API credentials (OAuth 2.0 Client ID)
4. Get channel ID from YouTube Studio
5. Triggers:
   - **Super Chat**: Amount threshold triggers effects
   - **Channel Memberships**: Triggers jumpscare

### Current Status:
- UI exists in dashboard
- Polling stub in place
- Needs: liveChatId retrieval, proper API polling, Super Chat parsing

---

## Kick Integration 🚧 (Stub - Needs Implementation)

### What's needed:
1. **Kick API** or **Pusher WebSocket** connection
2. Channel event subscription
3. Message parsing for subs/donations

### Setup (when implemented):
1. Get channel slug from Kick profile
2. Connect via Pusher WebSocket protocol
3. Subscribe to channel events
4. Triggers:
   - **Subscriptions**: Triggers jumpscare
   - **Donations**: Configurable threshold

### Current Status:
- UI exists in dashboard
- Connection stub in place
- Needs: Pusher client integration, event parsing

---

## How to Complete Integrations

### For YouTube:
```javascript
// Add to index.html after YouTube API is set up
async function getActiveLiveChat(channelId, apiKey) {
  const res = await fetch(
    `https://www.googleapis.com/youtube/v3/liveBroadcasts?part=snippet&broadcastStatus=active&channelId=${channelId}&key=${apiKey}`
  );
  const data = await res.json();
  return data.items[0]?.snippet?.liveChatId;
}

async function pollLiveChat(liveChatId, apiKey) {
  const res = await fetch(
    `https://www.googleapis.com/youtube/v3/liveChat/messages?liveChatId=${liveChatId}&part=snippet,authorDetails&key=${apiKey}`
  );
  const data = await res.json();
  // Parse for Super Chats, trigger effects
}
```

### For Kick:
```javascript
// Use pusher-js library
import Pusher from 'pusher-js';

const pusher = new Pusher('KICK_APP_KEY', {
  cluster: 'us2',
  wsHost: 'ws-us2.pusher.com',
});

const channel = pusher.subscribe(`channel.${channelId}`);
channel.bind('subscription', (data) => {
  fireEffect('jumpscare');
});
```

---

## Contributing

If you implement YouTube or Kick support, please submit a PR! This is MIT licensed.
