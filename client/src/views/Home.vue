<template>
  <section class="bg-red-200 text-green-900 relative">
    <div class="hero-image bg-right-bottom bg-cover flex">
      <div class="relative container mx-auto p-4 flex items-end z-10">
        <div>
          <div class="content float-left py-4 px-5 my-5">
            <div class="heading mb-3 text-2xl md:text-4xl">
              Remember everything you still need to do!
            </div>
            <div class="text leading-normal hidden sm:block">
              Photo by
              <a
                href="https://unsplash.com/@laurensauderstudio?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
                >Lauren Sauder</a
              >
              on
              <a
                href="https://unsplash.com/s/photos/todo?utm_source=unsplash&utm_medium=referral&utm_content=creditCopyText"
                >Unsplash</a
              >
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "Home",
  components: {},
});
</script>

<style lang="scss" scoped>
.clear-left {
  clear: left;
}

.hero-image {
  background-image: url("../assets/lauren-sauder-6GPBVYLapYQ-unsplash.jpg");
  height: 75vh;
  max-height: 75vh;
}

.hero-image::after {
  display: block;
  position: absolute;
  background-image: linear-gradient(
    to bottom,
    rgba(255, 255, 255, 0) 0,
    #fff 100%
  );
  bottom: 0;
  left: 0;
  right: 0;
  height: 75vh;
  width: 100%;
  content: "";
}
</style>
