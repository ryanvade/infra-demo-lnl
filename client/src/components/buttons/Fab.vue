<template>
  <button
    class="fixed right-6 bottom-12 p-0 w-16 h-16 bg-gray-800 rounded-full hover:bg-gray-700 active:shadow-lg mouse shadow transition ease-in duration-200 focus:outline-none text-white"
  >
    <i class="fas fa-plus"></i>
  </button>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  setup(props, { emit }) {
    function click(event: MouseEvent) {
      emit("click", event);
    }

    return {
      click,
    };
  },
});
</script>
