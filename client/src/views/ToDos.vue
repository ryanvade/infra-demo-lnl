<template>
  <div>
    <div v-if="!loading" class="container mx-auto px-8">
      <h3 v-if="todos.length < 1" class="text-center mt-4">No Todos.</h3>
      <to-do
        v-for="todo of todos"
        :todo="todo"
        :key="todo.id"
        v-on:deleteTodo="deleteToDoTriggered"
      ></to-do>
    </div>
    <fab v-if="!loading" v-on:click="fabClick"></fab>
    <modal v-if="showCreateModal">
      <to-do-form v-on:close="fabClick" v-on:submit="submit"></to-do-form>
    </modal>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import { useTodos } from "@/composables/todos";
import ToDo from "@/components/todos/ToDo.vue";
import Fab from "@/components/buttons/Fab.vue";
import Modal from "@/components/Modal.vue";
import ToDoForm from "@/components/forms/ToDo.vue";

export default defineComponent({
  name: "ToDos",
  components: {
    ToDo,
    Fab,
    Modal,
    ToDoForm,
  },
  setup() {
    const { todos, loading, createTodo, deleteToDo } = useTodos();
    const showCreateModal = ref<boolean>(false);

    function fabClick() {
      showCreateModal.value = !showCreateModal.value;
    }

    async function submit(event: { descr: string }) {
      await createTodo(event.descr);
      showCreateModal.value = false;
    }

    async function deleteToDoTriggered(event: string) {
      await deleteToDo(event);
    }

    return {
      todos,
      loading,
      showCreateModal,
      fabClick,
      submit,
      deleteToDoTriggered,
      deleteToDo,
    };
  },
});
</script>
