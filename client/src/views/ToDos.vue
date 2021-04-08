<template>
  <div v-if="!loading">
    <to-do v-for="todo of todos" :todo="todo" :key="todo.id"></to-do>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { useTodos } from "@/composables/todos";
import ToDo from "@/components/todos/ToDo.vue";

export default defineComponent({
  name: "ToDos",
  components: {
    ToDo,
  },
  setup() {
    const { todos, loading } = useTodos();

    return {
      todos,
      loading,
    };
  },
});
</script>
