<template>
  <section class="min-h-screen px-4 bg-white" v-if="todo">
    <div class="max-w-lg w-full rounded-lg shadow-lg p-4">
      <h3 class="font-semibold text-lg text-gray-700 tracking-wide">
        Created on {{ createdAt }}
      </h3>
      <p class="text-gray-500 my-1">{{ todo.descr }}</p>
    </div>
  </section>
</template>

<script lang="ts">
import { defineComponent, computed } from "vue";

export default defineComponent({
  props: {
    todo: Object,
  },
  setup(props) {
    const createdAt = computed<string>(() => {
      if (!props.todo) {
        return "";
      }
      const d = new Date(props.todo.createdAt);
      const dateString = d.toLocaleDateString();
      const timeString = d.toLocaleTimeString();
      return `${dateString} ${timeString}`;
    });
    return {
      createdAt,
    };
  },
});
</script>
