<template>
  <form
    class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4"
    v-on:submit.prevent="submit"
    :disabled="disabled"
  >
    <div class="sm:flex sm:items-start">
      <div
        class="mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-gray-100 sm:mx-0 sm:h-10 sm:w-10 text-gray-600"
      >
        <i class="fas fa-sticky-note"></i>
      </div>
      <div class="mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left w-full">
        <h3
          class="text-lg leading-6 font-medium text-gray-900"
          id="modal-title"
        >
          Create To Do
        </h3>
        <div class="mt-2 w-full">
          <label class="block">
            <span class="text-gray-700">Description</span>
            <textarea
              class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
              rows="3"
              v-model="descr"
              v-on:blur="validateDescription"
              :disabled="disabled"
            ></textarea>
            <span v-if="errors.descr" class="text-red-500">{{
              errors.descr
            }}</span>
          </label>
        </div>
      </div>
    </div>
    <div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
      <button
        type="submit"
        class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-blue-400 text-base font-medium text-white hover:bg-blue-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:ml-3 sm:w-auto sm:text-sm"
        :disabled="disabled"
      >
        Create
      </button>
      <button
        type="button"
        class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"
        v-on:click="close"
        :disabled="disabled"
      >
        Cancel
      </button>
    </div>
  </form>
</template>

<script lang="ts">
import { defineComponent, reactive, toRefs, ref } from "vue";

export default defineComponent({
  emits: ["submit", "close"],
  setup(props, { emit }) {
    const form = reactive<{ descr: string }>({
      descr: "",
    });

    const errors = reactive<{ descr: string | null }>({
      descr: null,
    });

    const disabled = ref<boolean>(false);

    function submit() {
      if (form.descr === "") {
        errors.descr = "Description is required.";
        return;
      }
      disabled.value = true;
      emit("submit", form);
    }

    function close() {
      emit("close");
    }

    function validateDescription() {
      if (form.descr !== "") {
        errors.descr = null;
      } else {
        errors.descr = "Description is required.";
      }
    }


    return {
      ...toRefs(form),
      errors,
      close,
      submit,
      validateDescription,
      disabled,
    };
  },
});
</script>
