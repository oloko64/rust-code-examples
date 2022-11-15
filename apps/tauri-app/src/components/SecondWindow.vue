<template>
  <div class="container">
    <h1>Second page</h1>
    <div>
      <button @click="OpenFileSelector">Open first window</button>
      <img src="../assets/teste_vue-tauri.jpeg" alt="" width="600" />
    </div>
  </div>
</template>
<script lang="ts" setup>
import { open } from "@tauri-apps/api/dialog";

async function OpenFileSelector() {
  const selected = await open({
    multiple: true,
    filters: [
      {
        name: "Image",
        extensions: ["png", "jpeg"],
      },
    ],
  });
  if (Array.isArray(selected)) {
    alert(selected.join(", "));
  } else if (selected === null) {
    alert("No file selected");
  } else {
    alert(selected);
  }
}
</script>
