<script setup lang="ts">
</script>

<template>
  <button class="button-container" @click="$router.push('/api/login')">
    <div class="upper-hitbox" />
    <div class="button-upper">
      <p>Login</p>
    </div>
    <div class="button-height" />
  </button>
</template>

<style scoped>
button {
    margin: 0;
    padding: 0;

    background: none;
    border: none;
    outline: none;
    box-shadow: none;
}

.button-container {
  background: none;
}

.button-container:hover {
  cursor: pointer;
}

.button-container:hover .button-upper {
  border-color: rgb(156, 155, 155);
  border-style: solid;
  border-width: 3px 3px 0px 3px;

  transition: 0.2s
}

.button-container:hover .button-height {
  border-color: rgb(156, 155, 155);
  border-style: solid;
  border-width: 0px 3px 3px 3px;

  transition: 0.2s
}

.button-container:active .upper-hitbox {
  height: 14px;

  transition: 0.2s
}

.button-container:active .button-upper {
  box-shadow: inset 0px 0px 0px 3px #72ca54;

  border-color: rgb(196, 196, 196);
  border-style: solid;
  border-width: 3px 3px 0px 3px;


  transition: 0.2s
}

.button-container:active .button-height {
  height: 0px;

  border-color: rgb(196, 196, 196);
  border-style: solid;
  border-width: 0px 3px 3px 3px;

  transition: 0.2s
}

.upper-hitbox {
    height: 0px;
    transition: height 0.2s;
}

.button-upper {
    width: 400px;
    height: 56px;
  
    display: flex;
    justify-content: center;
    align-items: center;
  
    background-color: #52a535;
  
    border-color: rgb(30, 30, 31);
    border-style: solid;
    border-width: 3px 3px 0px 3px;
  
    box-shadow: inset 0px 0px 0px 3px #64b14a;
  
    transition: 0.2s
  }

  .button-upper p {
    font-family: "Minecraft Five v2";
    font-size: 22px;
  
    text-align: center;
    text-shadow: 0px 2px black;
  
    color: white;
  }

  .button-height {
    width: 400px;
    height: 14px;
  
    background-color: #1d4d13;
  
    border-color: rgb(30, 30, 31);
    border-style: solid;
    border-width: 0px 3px 3px 3px;
  
    transition: 0.2s
  }
</style>