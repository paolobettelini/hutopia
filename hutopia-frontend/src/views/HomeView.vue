<script setup>
import { defineExpose } from 'vue';
import LoginButton from '../components/LoginButton.vue'

function loadServer() {
  let inputUrl = document.getElementById('urlInput').value;
  let iframe = document.createElement('iframe');
  iframe.src = inputUrl + '/space_file/index.html';
  iframe.width = '100%';
  iframe.height = '400px';
  iframe.style.border = '1px solid #ccc';
  let iframeContainer = document.getElementById('iframeContainer');
  iframeContainer.innerHTML = ''; // Clear previous iframe
  iframeContainer.appendChild(iframe);
}

defineExpose({ loadServer });
</script>

<template>
  <LoginButton/>

  <p> Home page </p>
  <input type="text" id="urlInput" placeholder="Enter URL"/>
  <button @click="loadServer">Load Server</button>

  <div id="iframeContainer"></div>
</template>
