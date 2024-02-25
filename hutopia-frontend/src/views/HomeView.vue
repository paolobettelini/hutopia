<script setup lang="ts">
import { onMounted, ref } from 'vue'
import LoginButton from '../components/LoginButton.vue';
import postData from "@/utils/post.js";

// TODO: put this stuff in a Component

function loadServer(): void {
  const inputUrl = (document.getElementById('urlInput') as HTMLInputElement).value;
  const iframe = document.createElement('iframe');

  const iframeContainer = document.getElementById('iframeContainer');
  if (iframeContainer) {
    iframeContainer.innerHTML = ''; // Clear previous iframe
    iframeContainer.appendChild(iframe);
  } else {
    console.error('iframeContainer not found.');
  }

  const storedCookies = JSON.parse(localStorage.getItem(inputUrl) || '{}');
  if (storedCookies && Date.now() < storedCookies.expiry) {
    // If cookies exist in localStorage and not expired, use them
    console.log("[SPACE AUTH] - Using cookie in local storage");
    iframe.contentWindow.document.cookie = `username=${storedCookies.username}; path=/`;
    iframe.contentWindow.document.cookie = `token=${storedCookies.token}; path=/`;
    loadIframe(inputUrl, iframe);
  } else {
    // If cookies don't exist in localStorage or expired, do the postData query
    console.log("[SPACE AUTH] - Generating token for auth from relay server");
    postData(`/genSpaceAuthToken`)
      .then(response => response.json())
      .then(json => {
        // Set username and token cookies for space authentication
        let token = json.token;
        let username = json.username;
        iframe.contentWindow.document.cookie = `username=${username}; path=/`;
        iframe.contentWindow.document.cookie = `token=${token}; path=/`;

        // Store cookies in localStorage with expiration duration of 1 day
        const expiry = Date.now() + 24 * 60 * 60 * 1000 - 10000; // 1 day
        localStorage.setItem(inputUrl, JSON.stringify({ username, token, expiry }));

        loadIframe(inputUrl, iframe);
      });
  }
}

// TODO: if the space doesn't authenticate you, it may be because the space restarted
// and lost the token from its RAM, so in that case we should remove the cookie
// from the local storage.

function loadIframe(inputUrl: string, iframe: HTMLIFrameElement): void {
  iframe.src = inputUrl + '/space_file/index.html';
  iframe.width = '100%';
  iframe.height = '400px';
  iframe.style.border = '1px solid #ccc';
}

const user = ref(null);

// User data json
onMounted(async () => {
  let response = await postData(`/userData`);
  user.value = await response.json();
})

defineExpose({ loadServer });
</script>

<template>
  <Suspense>
    <template #default>
      <div v-if="user">
          <div v-if="!user.logged">
            <p>You need to log in</p>
            <!-- <LoginButton />-->
            <a href="/api/login">Sign in / Sign up with google</a>
          </div>

          <div v-if="user.logged">
            <p>Welcome {{ user.username }}, your mail is {{ user.email }}</p>

            <input type="text" id="urlInput" placeholder="Enter URL"/>
            <button @click="loadServer">Load Server</button>

            <div id="iframeContainer"></div>

            <br>
            <a href="/api/logout">Logout</a>
          </div>
      </div>
      <div v-else>
        Loading...
      </div>
    </template>
  </Suspense>
</template>
