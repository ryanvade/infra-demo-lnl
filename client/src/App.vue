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

<style lang="scss">
    $fa-font-path : "~@fortawesome/fontawesome-free-webfonts/webfonts";
    @import "~@fortawesome/fontawesome-free-webfonts/scss/fontawesome.scss";
    @import "~@fortawesome/fontawesome-free-webfonts/scss/fa-solid.scss";
    @import "~@fortawesome/fontawesome-free-webfonts/scss/fa-regular.scss";
    @import "~@fortawesome/fontawesome-free-webfonts/scss/fa-brands.scss";
@import "~@fortawesome/fontawesome-free/scss/fontawesome";
@import "~@fortawesome/fontawesome-free/scss/brands";
@import "~@fortawesome/fontawesome-free/scss/regular";
@import "~@fortawesome/fontawesome-free/scss/solid";
</style>
