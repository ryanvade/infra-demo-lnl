<template>
  <section class="px-4 bg-white w-full mt-4" v-if="todo">
    <div class=" w-full rounded-lg shadow-lg p-4 flex">
      <div class="flex-grow">
      <h3 class="font-semibold text-lg text-gray-700 tracking-wide">
        Created on {{ createdAt }}
      </h3>
      <p class="text-gray-500 my-1">{{ todo.descr }}</p>
      </div>
      <button class="float-right" v-on:click="deleteTodo"><i class="fas fa-trash"></i></button>
    </div>
  </section>
</template>

<script lang="ts">
import { defineComponent, computed } from "vue";

export default defineComponent({
  props: {
    todo: Object,
  },
  setup(props, { emit }) {
    const createdAt = computed<string>(() => {
      if (!props.todo) {
        return "";
      }
      const d = new Date(props.todo.createdAt);
      const dateString = d.toLocaleDateString();
      const timeString = d.toLocaleTimeString();
      return `${dateString} ${timeString}`;
    });

    function deleteTodo() {
      emit("deleteTodo", props.todo?.id);
    }

    return {
      createdAt,
      deleteTodo,
    };
  },
});
</script>
