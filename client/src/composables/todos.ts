/* eslint-disable @typescript-eslint/explicit-module-boundary-types */
import { ToDo } from "@/domain";
import { reactive, onMounted, toRefs } from "vue";
import { useAuth } from "@/composables/auth";
import axios from "axios";

export interface ToDoState {
  todos: Array<ToDo>;
  loading: boolean;
}

const state = reactive<ToDoState>({
  todos: [],
  loading: true,
});

export function useTodos() {
  const { getAccessToken, authenticated } = useAuth();
  const instance = axios.create();
  instance.interceptors.request.use(async (config) => {
    const token = await getAccessToken();
    config.headers.get["Authorization"] = `Bearer ${token}`;
    config.headers.post["Authorization"] = `Bearer ${token}`;
    config.headers.patch["Authorization"] = `Bearer ${token}`;
    config.headers.delete["Authorization"] = `Bearer ${token}`;
    return config;
  });

  async function getTodos() {
    state.loading = true;
    const url = `${process.env.VUE_APP_API_URL}/api/todos`;
    const response = await instance.get<{ items: Array<ToDo> }>(url);
    const todos = response.data.items;
    state.todos = todos;
    state.loading = false;
    return response.data.items;
  }

  async function createTodo(descr: string) {
    state.loading = true;
    const url = `${process.env.VUE_APP_API_URL}/api/todos`;
    await instance.post(url, { descr });
    const todos = getTodos();
    return todos;
  }

  async function deleteToDo(id: string) {
    state.loading = true;
    const url = `${process.env.VUE_APP_API_URL}/api/todos/${id}`;
    await instance.delete(url);
    const todos = getTodos();
    return todos;
  }

  onMounted(async () => {
    if (authenticated.value) {
      await getTodos();
    }
  });

  return {
    ...toRefs(state),
    getTodos,
    createTodo,
    deleteToDo,
  };
}
