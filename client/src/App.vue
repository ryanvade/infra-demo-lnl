<template>
  <nav-bar></nav-bar>
  <div v-if="!authenticating">
    <router-view />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useAuthInit } from "@/composables/auth";
import NavBar from "@/components/navigation/NavBar.vue";

export default defineComponent({
  name: "App",
  components: {
    NavBar,
  },
  setup() {
    const { authenticating } = useAuthInit();

    return {
      authenticating,
    };
  },
});
</script>

<style lang="scss"></style>
