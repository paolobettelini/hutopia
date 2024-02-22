<script setup lang="ts">
import { onMounted, ref } from 'vue'
import LoginButton from '../components/LoginButton.vue';
import postData from "@/utils/post.js";

function loadServer(): void {

  //
  const inputUrl = (document.getElementById('urlInput') as HTMLInputElement).value;
  const iframe = document.createElement('iframe');
  iframe.src = inputUrl + '/space_file/index.html';
  iframe.width = '100%';
  iframe.height = '400px';
  iframe.style.border = '1px solid #ccc';
  const iframeContainer = document.getElementById('iframeContainer');
  if (iframeContainer) {
    iframeContainer.innerHTML = ''; // Clear previous iframe
    iframeContainer.appendChild(iframe);
  } else {
    console.error('iframeContainer not found.');
  }
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
